
bindir:
	mkdir -p bin/wasm

wasm:
	make bindir
	cargo build --release --target wasm32-unknown-unknown
	cp \
		target/wasm32-unknown-unknown/release/quoridor-rust.wasm \
		src/wasm-release/* \
		bin/wasm/

run_wasm:
	make wasm
	cd bin/wasm && basic-http-server .
