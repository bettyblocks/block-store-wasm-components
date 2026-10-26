# block-store-wasm-components

WebAssembly components for the Betty Blocks block store functions (concat
text, format endpoint result, generate UUID, liquid template, ...). Each
function is a Rust crate under `functions/<name>/<version>/` that compiles to a
`wasm32-wasip2` component. Its interface is `wit/world.wit`, and the function
definition the block store shows is `function.json`.

The components are published to the Betty Blocks Azure container registries,
where the platform pulls them by their WIT package name and version.

## Requirements

- Rust with the `wasm32-wasip2` target (`rustup target add wasm32-wasip2`)
- [`just`](https://github.com/casey/just)
- [`wkg`](https://github.com/bytecodealliance/wasm-pkg-tools): the shared
  `build.rs` runs `wkg wit fetch` for every component to download its WIT
  dependencies into `wit/deps/` (gitignored). CI installs wkg 0.15.1.

## Commands

```sh
just build          # build every component and copy its .wasm next to its Cargo.toml
just test           # build, then run the tests (cargo test)
just format-check   # cargo fmt --check
just quality-check  # cargo clippy --all-targets -- -D warnings
```

CI runs these with `RUSTFLAGS="--deny warnings"`.

## Registry config: `wkg-config.toml`

`wkg-config.toml` maps WIT namespaces to registries (`wasi`, `wasmcloud`, and
`betty-blocks-types`, which lives in the Azure registry). The shared `build.rs`
passes it to `wkg` as `WKG_CONFIG_FILE`, unless `WKG_CONFIG_FILE` is already
set, so you can point a build at another config.

It is deliberately not called `wkg.toml`: wkg 0.16 and later look for a
`wkg.toml` in parent folders and parse it as a project manifest, not as a
registry config.

The committed file points `betty-blocks-types` at the dev registry
(`bettyblocksdev.azurecr.io`). The release workflow rewrites that line per
environment.

A failed `wkg wit fetch` fails the build. It does not fall back to WIT deps
left on disk.

## Versions

The version that counts is the package version on the first line of each
component's `wit/world.wit`:

```wit
package betty-blocks:concat-text@1.0.0;
```

It is the tag the component is published under. It is independent of the
`Cargo.toml` version and of the `functions/<name>/<version>/` directory.

**Every PR that changes a component must raise that version.** The
`Version Check` workflow runs `scripts/check-version-bumps.sh` on every PR. A
component counts as changed when its `src/`, `Cargo.toml`, `build.rs` or
`wit/` (excluding `wit/deps/`) differs from `main`. Then:

- the version is unchanged → fails;
- the version went down → fails;
- any increase (major, minor or patch) → passes; treat a breaking change as a
  new major;
- a component that does not exist on `main` yet may start at any version.

Shared files at the repository root (`build.rs`, `Cargo.toml`, `Cargo.lock`,
`wkg-config.toml`, `Justfile`) and `tests/` do not count as a change to any
component.

Run it locally with `./scripts/check-version-bumps.sh` (it compares against
`origin/main`).

## Release flow

`main` is the only branch. A merge to `main` runs `release.yaml`:

1. CI (`ci.yml`) runs first. Nothing is published unless it passes.
2. For each GitHub environment, `dev` (Azure `bettyblocksdev`, which serves
   edge and acceptance) and `main` (Azure prod), the workflow points
   `wkg-config.toml` at that environment's registry, builds all components and
   logs in to the registry.
3. Each component is published as
   `<registry>/betty-blocks/<package>:<world.wit version>`, but only when that
   tag does not exist yet (checked with `scripts/tag-exists.sh`). Existing tags
   are skipped and logged, never overwritten: the runtime precompiles each tag
   once, so a new build under an old tag would not reach deployed apps.

To ship a change, bump the version in `wit/world.wit`; the Version Check makes
sure you do.
