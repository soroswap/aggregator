default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/comet_adapter.wasm
	@rm ../../target/wasm32-unknown-unknown/release/comet_adapter.wasm

fmt:
	cargo fmt --all --check

clean:
	cargo clean