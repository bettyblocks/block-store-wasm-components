import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/datetime/1.0/datetime.wasm";

const {
  datetime: {
    now,
    changeTimezone,
    offsetDatetime,
    offsetDatetimeInBusinessDays,
  },
} = await compileComponent(wasmPath);

test("now works", () => {
  const result = now();

  expect(Date.now() - new Date(result).getTime()).toBeLessThan(1000);
});

test("change time zone works", () => {
  const result = changeTimezone("1970-01-01T00:00:00+00:00", "-05:00");

  expect(result.substring(result.length - 6, result.length)).toBe("-05:00");
  expect(new Date(result).getUTCHours()).toBe(0);
});

test("offset datetime works", () => {
  const result = offsetDatetime("1970-01-01T00:00:00+00:00", 2, "hours");

  expect(new Date(result).getUTCHours()).toBe(2);
});

test("offset datetime in business days works", () => {
  const result = offsetDatetimeInBusinessDays(
    "1970-01-01T00:00:00+00:00",
    2,
    "days",
  );

  expect(new Date(result).getUTCDate()).toBe(5);
});
