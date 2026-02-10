import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/liquid/1.0/liquid_templating.wasm";

const { liquid: { liquid } } = await compileComponent(wasmPath);

test("it works", () => {
  const renderedTemplate = liquid(
    "hi {{something}}",
    undefined,
    '{ "something": "value" }',
  );
  expect(renderedTemplate).toBe("hi value");
});

test("it gives template_variable precedence over template", () => {
  const renderedTemplate = liquid(
    "hi",
    "hi {{something}}",
    '{ "something": "value" }',
  );
  expect(renderedTemplate).toBe("hi value");
});

test("it does not work with no JSON object", () => {
  let result;
  try {
    liquid("This parameter does not matter", undefined, "");
  } catch (error) {
    result = error;
  }
  expect(result).not.toBe(undefined);
});
