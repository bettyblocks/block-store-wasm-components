fetch_wit_deps:
	wkg wit fetch

# mv will return a non-zero exit code if trying to move a file to a copy of itself.
# Meaning, `./target/wasm32-wasip2/release/something.wasm` is the exact same as `./something.wasm`, making mv error.
# We surpress it here as this is not an error in our case.
move_wasm_to_root: 
	mv ./target/wasm32-wasip2/release/*.wasm . 2> /dev/null || exit 0

build: fetch_wit_deps
	cargo build --release --target wasm32-wasip2
	just move_wasm_to_root

test: 
	cargo test

format:
	cargo fmt

format-check:
	cargo fmt --check

quality-check:
	cargo clippy

clean:
	cargo clean
