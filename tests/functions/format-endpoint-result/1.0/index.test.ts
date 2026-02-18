import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath =
  "./functions/format-endpoint-result/1.0/format_endpoint_result.wasm";

const { formatEndpointResult: { formatEndpointResult } } =
  await compileComponent(wasmPath);

test("it works", () => {
  const result = formatEndpointResult(200, "{result: true}", [{
    key: "Accept",
    value: "application/json",
  }]);

  expect(Array.isArray(result.headers[0])).toBe(true);
});
