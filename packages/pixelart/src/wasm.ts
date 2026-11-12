import init, { initSync } from "../pkg/renderer_core.js";

const nodeProcess = (globalThis as typeof globalThis & { process?: { versions?: { node?: string } } }).process;
const wasmUrl = new URL("../pkg/renderer_core_bg.wasm", import.meta.url);

if (nodeProcess?.versions?.node) {
  const { readFile } = await import("node:fs/promises");
  initSync({ module: await readFile(wasmUrl) });
} else {
  await init({ module_or_path: wasmUrl });
}
