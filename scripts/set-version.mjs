import { readFile, writeFile } from "node:fs/promises";

const version = process.argv[2];
if (process.argv[1]?.endsWith("set-version.mjs")) {
  if (!version || !/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(version)) {
    throw new Error("Usage: node scripts/set-version.mjs <semver>");
  }
  await setVersion(version);
}

export async function setVersion(version) {
  if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(version)) {
    throw new Error(`Invalid semantic version: ${version}`);
  }

  async function updateJson(path, mutate) {
    const json = JSON.parse(await readFile(path, "utf8"));
    mutate(json);
    await writeFile(path, `${JSON.stringify(json, null, 2)}\n`);
  }

  await updateJson("package.json", (json) => { json.version = version; });
  await updateJson("packages/pixelart/package.json", (json) => { json.version = version; });
  await updateJson("package-lock.json", (json) => {
    json.version = version;
    json.packages[""].version = version;
    json.packages["packages/pixelart"].version = version;
  });

  for (const path of ["apps/cli/Cargo.toml", "apps/mcp/Cargo.toml"]) {
    const toml = await readFile(path, "utf8");
    const updated = toml.replace(/^(version\s*=\s*)"[^"]+"/m, `$1"${version}"`);
    if (updated === toml) throw new Error(`Could not find package version in ${path}`);
    await writeFile(path, updated);
  }

  const lockPath = "Cargo.lock";
  const lock = await readFile(lockPath, "utf8");
  const blocks = lock.split(/(?=^\[\[package\]\]$)/m);
  const expected = new Set(["openpixel-cli", "openpixel-mcp"]);
  const found = new Set();
  const updatedBlocks = blocks.map((block) => {
    const name = block.match(/^name\s*=\s*"([^"]+)"$/m)?.[1];
    if (!expected.has(name)) return block;
    const updated = block.replace(/^(version\s*=\s*)"[^"]+"/m, `$1"${version}"`);
    if (updated === block) throw new Error(`Could not update ${name} in Cargo.lock`);
    found.add(name);
    return updated;
  });
  if (found.size !== expected.size) {
    throw new Error(`Cargo.lock missing expected packages: ${[...expected].filter((name) => !found.has(name)).join(", ")}`);
  }
  await writeFile(lockPath, updatedBlocks.join(""));
}
