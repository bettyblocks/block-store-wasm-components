import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/redirect/1.0/redirect.wasm";

const { redirect: { redirect } } = await compileComponent(wasmPath);

test("it works", () => {
  const result = redirect("http://example.com");

  expect(result.statusCode).toBe(302);
  expect(result.body).toBe("Redirect");
  expect(result.headers).toEqual([["Location", "http://example.com"]]);
});
