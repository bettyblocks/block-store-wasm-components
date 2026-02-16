import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/split-text/1.0/split_text.wasm";

const { splitText: { splitAll } } = await compileComponent(wasmPath);

test("it can split all", () => {
  const splits = splitAll("hi hi hi hi", " ");
  expect(splits).toEqual(["hi", "hi", "hi", "hi"]);
});

/* Commented out until tuples are supported.
test("it can split once", () => {
  const splits = splitOnce("hi hi hi hi", " ");
  expect(splits).toEqual(["hi", "hi hi hi"]);
});
*/
