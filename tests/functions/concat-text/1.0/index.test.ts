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
  const result = concatStrings("hi", "hooo");
  expect(result).toEqual("hihooo");
});

test("it concats strings with a separator", () => {
  const result = concatStringsWithSeparator("hi", "hi", "   ");
  expect(result).toEqual("hi   hi");
});

test("it concats string lists", () => {
  const result = concatStringList(["hi", "ha", "hi", "123"]);
  expect(result).toEqual("hihahi123");
});

test("it concats string lists with a separator", () => {
  const result = concatStringListWithSeparator([
    "something",
    "what",
    "hello",
    "22!",
  ], " ");
  expect(result).toEqual("something what hello 22!");
});
