BINDIR="bin/"
WASM_DIR="${BINDIR}/wasm"


bindir:
	mkdir -p "${WASM_DIR}"

wasm:
	make bindir
	cargo build --release --target wasm32-unknown-unknown
	cp \
		target/wasm32-unknown-unknown/release/quoridor-rust.wasm \
		src/wasm-release/* \
		${WASM_DIR}/

run_wasm:
	make wasm
	cd ${WASM_DIR} && basic-http-server .

build_dev:
	cargo build --profile dev

build_release:
	cargo build --profile release
