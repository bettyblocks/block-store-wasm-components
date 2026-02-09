import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/concat-text/1.0/concat_text.wasm";

const { concatText: { concatStrings, concatStringsWithSeparator } } =
  await compileComponent(wasmPath);

test("it concat strings", () => {
  const result = concatStrings(["hi", "hi", "hi", "hi"]);
  expect(result).toEqual("hihihihi");
});

test("it concat strings with a separator", () => {
  const result = concatStringsWithSeparator(["hi", "hi", "hi", "hi"], " ");
  expect(result).toEqual("hi hi hi hi");
});
