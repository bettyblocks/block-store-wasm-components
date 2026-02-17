import { transpile } from "@bytecodealliance/jco-transpile";
import { URL } from "node:url";
import { Blob } from "node:buffer";
import { WASIShim } from "@bytecodealliance/preview2-shim/instantiation";

const wasishim = new WASIShim();

async function compileComponent(wasm: string) {
  const { files } = await transpile(wasm, {
    outDir: "",
    nodejsCompat: false,
    emitTypescriptDeclarations: false,
  });

  const wrapperScriptFile = Object.keys(files).find((x) => x.endsWith(".js"));
  if (!wrapperScriptFile) {
    throw new Error("no js file found");
  }

  const blob = new Blob([files[wrapperScriptFile]!]);
  const blobUrl = URL.createObjectURL(blob);
  const jsFunctions = await import(blobUrl);

  return jsFunctions.instantiate(
    (path: string) =>
      new WebAssembly.Module(files[path]! as unknown as ArrayBuffer),
    wasishim.getImportObject(),
  );
}

export { compileComponent };
