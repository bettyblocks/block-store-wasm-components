import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/generate-uuid/1.0/generate_uuid.wasm";

const { generateUuid: { generateUuid } } = await compileComponent(wasmPath);

test("it works", () => {
  const { uuid } = generateUuid();
  // deno bug, we patched it in compile-component.ts
  expect(uuid).not.toBe("00000000-0000-4a0d-8000-000000000a0d");

  expect(uuid).toHaveLength(36);
});
