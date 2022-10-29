

build:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/xp4_istio_wasme_rust.wasm .
	wasme build precompiled xp4_istio_wasme_rust.wasm -t webassemblyhub.io/dxps/xp4_istio_wasme_rust:$(tag)

publish: build
	wasme push webassemblyhub.io/dxps/xp4_istio_wasme_rust:$(tag)

all: build publish

