import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/random-hex/1.0/random_hex.wasm";

const { randomHex: { generateRandomHex } } = await compileComponent(wasmPath);

test("it works", () => {
  const randomHex = generateRandomHex(20);

  expect(parseInt(randomHex, 16)).toBeGreaterThan(0);

  expect(randomHex).toHaveLength(20);
});
