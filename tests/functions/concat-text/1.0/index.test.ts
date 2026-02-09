import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/concat-text/1.0/concat_text.wasm";

const {
  concatText: {
    concatStrings,
    concatStringsWithSeparator,
    concatStringList,
    concatStringListWithSeparator,
  },
} = await compileComponent(wasmPath);

test("it concats strings", () => {
  const result = concatStrings("hi", "hi");
  expect(result).toEqual("hihi");
});

test("it concats strings with a separator", () => {
  const result = concatStringsWithSeparator("hi", "hi", " ");
  expect(result).toEqual("hi hi");
});

test("it concats string lists", () => {
  const result = concatStringList(["hi", "hi", "hi", "hi"]);
  expect(result).toEqual("hihihihi");
});

test("it concats string lists with a separator", () => {
  const result = concatStringListWithSeparator(["hi", "hi", "hi", "hi"], " ");
  expect(result).toEqual("hi hi hi hi");
});
