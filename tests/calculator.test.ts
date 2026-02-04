import { compileComponent } from "./compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const { calculator: { add, subtract, divide, multiply } } =
  await compileComponent(
    "./tests/calculator.wasm",
  );

test("it adds numbers", () => {
  expect(add(34, 35)).toBe(69);
});

test("it subtracts numbers", () => {
  expect(subtract(69, 35)).toBe(34);
});

test("it divides numbers", () => {
  expect(divide(28980, 420)).toBe(69);
});

test("it multiplies numbers", () => {
  expect(multiply(69, 420)).toBe(28980);
});
