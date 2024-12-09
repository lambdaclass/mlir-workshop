.PHONY: deps
deps:
	brew install -q llvm@19

build:
	cargo build

run:
	cargo run -- test.prog -o out.a
