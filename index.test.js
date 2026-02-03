import { transpile, writeFiles } from "@bytecodealliance/jco-transpile";
// import { expect, test } from "bun:test";
// import { expect, test } from "vitest";
const { test } = Deno;
import { expect } from "jsr:@std/expect";

async function compileComponent(wasm) {
  // const { transpile } = await import("@bytecodealliance/jco-transpile");
  const { files, imports, exports } = await transpile(wasm, {
    outDir: "",
    nodejsCompat: false,
    emitTypescriptDeclarations: false,
  });

  const requirable = Object.keys(files).find((x) => x.endsWith(".js"));
  if (!requirable) {
    throw new Error("no js file found");
  }

  const blob = new Blob([files[requirable]]);
  const blob_url = URL.createObjectURL(blob);
  const js_functions = await import(blob_url);

  return js_functions.instantiate(
    (path) => new WebAssembly.Module(files[path]),
  );
}

const { calculator: { add, subtract, divide, multiply } } =
  await compileComponent(
    "calculator.wasm",
  );

test("it adds numbers", async () => {
  expect(add(34, 35)).toBe(69);
});

test("it subtracts numbers", async () => {
  expect(subtract(69, 35)).toBe(34);
});

test("it divides numbers", async () => {
  expect(divide(28980, 420)).toBe(69);
});

test("it multiplies numbers", async () => {
  expect(multiply(69, 420)).toBe(28980);
});
