build:
	cargo build --release
	just distribute-wasm

distribute-wasm:
	#!/usr/bin/env bash
	for directory in $(just index); do
		library_name="$(grep '^name' "$directory/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/' | tr '-' '_')"
		wasm_path="target/wasm32-wasip2/release/${library_name}.wasm"
		if [ -f "$wasm_path" ]; then
			cp "$wasm_path" "$directory/"
		fi
	done

test:
	cargo test --workspace

format:
	cargo fmt --all

format-check:
	cargo fmt --all --check

quality-check:
	cargo clippy --workspace --all-targets -- -D warnings

clean:
	cargo clean

integration-test:
	deno install
	deno fmt --check
	deno lint
	deno task test

index:
	find functions -type f -name "Cargo.toml" -exec dirname {} \;
