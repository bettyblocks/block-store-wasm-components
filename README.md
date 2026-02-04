# block-store-wasm-components

## integration tests

Integration tests are written in typescript using Deno. It is using Deno because
it will just run typescript, has a dependency installer, formatter and linter
buildin.

I would prefer to have to use Bun, but the Bun test will always fail because of
some v8/libuv dependency https://github.com/bytecodealliance/jco/issues/1059

### format

```sh
deno fmt
```

### lint

```sh
deno lint
```

### run tests

```sh
deno task test
```
