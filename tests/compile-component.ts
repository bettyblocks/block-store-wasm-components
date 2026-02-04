import { transpile } from "@bytecodealliance/jco-transpile";
import { URL } from "node:url";
import { Blob } from "node:buffer";

async function compileComponent(wasm: string) {
  const { files } = await transpile(wasm, {
    outDir: "",
    nodejsCompat: false,
    emitTypescriptDeclarations: false,
  });

  const requirable = Object.keys(files).find((x) => x.endsWith(".js"));
  if (!requirable) {
    throw new Error("no js file found");
  }

  const blob = new Blob([files[requirable]!]);
  const blob_url = URL.createObjectURL(blob);
  const js_functions = await import(blob_url);

  return js_functions.instantiate(
    (path: string) =>
      new WebAssembly.Module(files[path]! as unknown as ArrayBuffer),
  );
}

export { compileComponent };
