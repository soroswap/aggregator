default: build

all: test

test: build
	cargo test

build: soroswap_build
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroswap_proxy.wasm
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

soroswap_build:
	@cd ../../protocols/soroswap/contracts && make build

fmt:
	cargo fmt --all --check

clean:
	cargo clean