import { rm } from "node:fs/promises";
import { spawnSync } from "node:child_process";

const result = spawnSync("wasm-pack", [
  "build",
  "../renderer-core",
  "--target",
  "web",
  "--out-dir",
  "../pixelart/pkg",
  "--out-name",
  "renderer_core"
], { stdio: "inherit" });

if (result.error) throw result.error;
if (result.status !== 0) process.exit(result.status ?? 1);

await rm(new URL("../pkg/.gitignore", import.meta.url), { force: true });
