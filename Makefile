BINDIR="bin/"
WASM_DIR="${BINDIR}/wasm"
ASSET_DIR="${WASM_DIR}/assets"


bindir:
	mkdir -p "${WASM_DIR}"
	mkdir -p "${ASSET_DIR}"

wasm:
	make bindir
	cargo build --release --target wasm32-unknown-unknown
	cp \
		target/wasm32-unknown-unknown/release/quoridor-rust.wasm \
		src/wasm-release/* \
		${WASM_DIR}/
	cp assets/digital-7.ttf ${ASSET_DIR}/

run_wasm:
	make wasm
	cd ${WASM_DIR} && basic-http-server .

build_dev:
	cargo build --profile dev

build_release:
	cargo build --profile release
