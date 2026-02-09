import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/is-present/1.0/is_present.wasm";

const { isPresent: { isPresent } } = await compileComponent(wasmPath);

test("it works", () => {
  const presence = isPresent("1");
  expect(presence).toBe(true);
});
