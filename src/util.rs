use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    path::{Path, PathBuf},
    ptr::null_mut,
    sync::OnceLock,
};

use llvm_sys::{
    core::{
        LLVMContextCreate, LLVMContextDispose, LLVMDisposeMemoryBuffer, LLVMDisposeMessage,
        LLVMDisposeModule, LLVMGetBufferSize, LLVMGetBufferStart,
    },
    error::LLVMGetErrorMessage,
    prelude::LLVMMemoryBufferRef,
    target::{
        LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargetMCs,
        LLVM_InitializeAllTargets,
    },
    target_machine::{
        LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetMachine,
        LLVMDisposeTargetMachine, LLVMGetDefaultTargetTriple, LLVMGetHostCPUFeatures,
        LLVMGetHostCPUName, LLVMGetTargetFromTriple, LLVMRelocMode,
        LLVMTargetMachineEmitToMemoryBuffer, LLVMTargetRef,
    },
    transforms::pass_builder::{
        LLVMCreatePassBuilderOptions, LLVMDisposePassBuilderOptions, LLVMRunPasses,
    },
};
use melior::ir::Module;
use mlir_sys::mlirTranslateModuleToLLVMIR;
use tracing::trace;

#[derive(Debug, Clone, Copy)]
pub enum OptLevel {
    None,
    Less,
    Default,
    Aggressive,
}

impl From<u8> for OptLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Less,
            2 => Self::Default,
            _ => Self::Aggressive,
        }
    }
}

pub unsafe fn llvm_compile(module: &Module, optlevel: OptLevel) -> Vec<u8> {
    static INITIALIZED: OnceLock<()> = OnceLock::new();
    INITIALIZED.get_or_init(|| unsafe {
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
        tracing::debug!("initialized llvm targets");
    });

    let llvm_context = LLVMContextCreate();

    let op = module.as_operation().to_raw();

    trace!("starting mlir to llvm compilation");
    let llvm_module = mlirTranslateModuleToLLVMIR(op, llvm_context as *mut _) as *mut _;

    let mut null = null_mut();
    let mut error_buffer = &raw mut null;

    let target_triple = LLVMGetDefaultTargetTriple();
    let target_cpu = LLVMGetHostCPUName();
    let target_cpu_features = LLVMGetHostCPUFeatures();

    let mut target: MaybeUninit<LLVMTargetRef> = MaybeUninit::uninit();

    if LLVMGetTargetFromTriple(target_triple, target.as_mut_ptr(), error_buffer) != 0 {
        let error = CStr::from_ptr(*error_buffer);
        let err = error.to_string_lossy().to_string();
        LLVMDisposeMessage(*error_buffer);
        panic!("error getting target triple: {}", err);
    } else if !(*error_buffer).is_null() {
        LLVMDisposeMessage(*error_buffer);
        error_buffer = &raw mut null;
    }

    let target = target.assume_init();

    let machine = LLVMCreateTargetMachine(
        target,
        target_triple.cast(),
        target_cpu.cast(),
        target_cpu_features.cast(),
        match optlevel {
            OptLevel::None => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
            OptLevel::Less => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
            OptLevel::Default => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            OptLevel::Aggressive => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        },
        LLVMRelocMode::LLVMRelocPIC,
        LLVMCodeModel::LLVMCodeModelDefault,
    );

    let opts = LLVMCreatePassBuilderOptions();

    let opt = match optlevel {
        OptLevel::None => 0,
        OptLevel::Less => 1,
        OptLevel::Default => 2,
        OptLevel::Aggressive => 3,
    };
    let passes = CString::new(format!("default<O{opt}>"))
        .expect("only fails if the hardcoded string contains a null byte");

    trace!("starting llvm passes");
    let error = LLVMRunPasses(llvm_module, passes.as_ptr(), machine, opts);

    if !error.is_null() {
        let msg = LLVMGetErrorMessage(error);
        let msg = CStr::from_ptr(msg);
        panic!("{}", msg.to_string_lossy());
    }

    LLVMDisposePassBuilderOptions(opts);

    let mut out_buf: MaybeUninit<LLVMMemoryBufferRef> = MaybeUninit::uninit();

    trace!("starting llvm to object compilation");
    let ok = LLVMTargetMachineEmitToMemoryBuffer(
        machine,
        llvm_module,
        LLVMCodeGenFileType::LLVMObjectFile,
        error_buffer,
        out_buf.as_mut_ptr(),
    );

    if ok != 0 {
        let error = CStr::from_ptr(*error_buffer);
        let err = error.to_string_lossy().to_string();
        LLVMDisposeMessage(*error_buffer);
        panic!("{}", err);
    } else if !(*error_buffer).is_null() {
        LLVMDisposeMessage(*error_buffer);
    }

    let out_buf = out_buf.assume_init();

    let out_buf_start: *const u8 = LLVMGetBufferStart(out_buf).cast();
    let out_buf_size = LLVMGetBufferSize(out_buf);

    // keep it in rust side
    let data = std::slice::from_raw_parts(out_buf_start, out_buf_size).to_vec();

    LLVMDisposeMemoryBuffer(out_buf);
    LLVMDisposeTargetMachine(machine);
    LLVMDisposeModule(llvm_module);
    LLVMContextDispose(llvm_context);

    data
}

pub fn link_binary(objects: &[PathBuf], output_filename: &Path) -> std::io::Result<()> {
    let objects: Vec<_> = objects.iter().map(|x| x.display().to_string()).collect();
    let output_filename = output_filename.to_string_lossy().to_string();

    let args: Vec<_> = {
        #[cfg(target_os = "macos")]
        {
            let mut args = vec![
                "-L/usr/local/lib",
                "-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib",
            ];

            args.extend(objects.iter().map(|x| x.as_str()));

            args.extend(&["-o", &output_filename, "-lSystem"]);

            args
        }
        #[cfg(target_os = "linux")]
        {
            let (scrt1, crti, crtn) = {
                if file_exists("/usr/lib64/Scrt1.o") {
                    (
                        "/usr/lib64/Scrt1.o",
                        "/usr/lib64/crti.o",
                        "/usr/lib64/crtn.o",
                    )
                } else {
                    (
                        "/lib/x86_64-linux-gnu/Scrt1.o",
                        "/lib/x86_64-linux-gnu/crti.o",
                        "/lib/x86_64-linux-gnu/crtn.o",
                    )
                }
            };

            let mut args = vec![
                "-pie",
                "--hash-style=gnu",
                "--eh-frame-hdr",
                "--dynamic-linker",
                "/lib64/ld-linux-x86-64.so.2",
                "-m",
                "elf_x86_64",
                scrt1,
                crti,
            ];

            args.extend(&["-o", &output_filename]);

            args.extend(&[
                "-L/lib64",
                "-L/usr/lib64",
                "-L/lib/x86_64-linux-gnu",
                "-zrelro",
                "--no-as-needed",
                "-lc",
                "-O1",
                crtn,
            ]);

            args.extend(objects.iter().map(|x| x.as_str()));

            args
        }
        #[cfg(target_os = "windows")]
        {
            unimplemented!()
        }
    };

    let mut linker = std::process::Command::new("ld");
    let proc = linker.args(args.iter()).spawn()?;
    let output = proc.wait_with_output()?;
    tracing::debug!("Linker result: {:#?}", output);
    Ok(())
}
