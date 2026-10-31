import { execFileSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import { setVersion } from "./set-version.mjs";

const dryRun = process.argv.includes("--dry-run");
const branch = execFileSync("git", ["branch", "--show-current"], { encoding: "utf8" }).trim();
if (!["master", "alpha", "beta"].includes(branch)) {
  throw new Error(`Releases are only configured for master, alpha, and beta (current: ${branch || "detached HEAD"})`);
}

const tags = execFileSync("git", ["tag", "--list", "v*"], { encoding: "utf8" }).trim().split(/\r?\n/).filter(Boolean);
const stable = tags.map(parseTag).filter((tag) => tag && !tag.prerelease).sort(compareVersions).at(-1);
const baseVersion = stable ?? { major: 0, minor: 0, patch: 0, raw: "0.0.0" };
let previous;
if (branch === "master") {
  previous = stable;
} else {
  previous = tags.map(parseTag).filter((tag) => tag?.prerelease?.[0] === branch).sort(compareVersions).at(-1) ?? stable;
}

const range = previous ? `v${previous.raw}..HEAD` : "HEAD";
const log = execFileSync("git", ["log", range, "--format=%H%x1f%s%x1f%b%x1e"], { encoding: "utf8" });
const commits = log.split("\x1e").map((entry) => entry.trim()).filter(Boolean).map((entry) => {
  const [hash, subject, ...body] = entry.split("\x1f");
  return { hash, subject, body: body.join("\n") };
});

let level = 0;
for (const commit of commits) {
  const breaking = /(?:^|\n)BREAKING CHANGE:\s*\S/i.test(commit.body) || /^\w+(?:\([^)]*\))?!:/.test(commit.subject);
  if (breaking) { level = Math.max(level, 3); continue; }
  const type = commit.subject.match(/^([a-z]+)(?:\([^)]*\))?:/i)?.[1]?.toLowerCase();
  if (type === "feat") level = Math.max(level, 2);
  else if (["fix", "perf", "refactor"].includes(type)) level = Math.max(level, 1);
}
if (!level) {
  console.log("No release-worthy Conventional Commits found.");
  process.exit(0);
}

let next = { major: baseVersion.major, minor: baseVersion.minor, patch: baseVersion.patch };
const prereleaseBase = previous?.prerelease ? previous : undefined;
if (prereleaseBase && level <= 2) {
  next = { major: prereleaseBase.major, minor: prereleaseBase.minor, patch: prereleaseBase.patch };
} else if (level === 3) {
  next.major++; next.minor = 0; next.patch = 0;
} else if (level === 2) {
  next.minor++; next.patch = 0;
} else {
  next.patch++;
}

let version = `${next.major}.${next.minor}.${next.patch}`;
if (branch !== "master") {
  const sameBase = tags.map(parseTag).filter((tag) => tag && tag.prerelease?.[0] === branch && tag.major === next.major && tag.minor === next.minor && tag.patch === next.patch);
  const counter = sameBase.reduce((highest, tag) => Math.max(highest, Number(tag.prerelease[1]) || 0), 0) + 1;
  version += `-${branch}.${counter}`;
}

const tag = `v${version}`;
const notes = makeNotes(commits);
console.log(`Next release: ${tag}\n\n${notes}`);
if (dryRun) process.exit(0);

await setVersion(version);
const changelogPath = "CHANGELOG.md";
const changelog = await readFile(changelogPath, "utf8").catch(() => "# Changelog\n\nThis file is automatically updated for each release.\n\n---\n\n");
const [header, history = ""] = changelog.split(/\n---\n/, 2);
const date = new Date().toISOString().slice(0, 10);
const entry = `## [${tag}] - ${date}\n\n${notes}`;
await writeFile(changelogPath, `${header}\n\n---\n\n${entry}${history ? `\n\n---\n\n${history.trimStart()}` : "\n"}`);
execFileSync("git", ["add", "CHANGELOG.md", "Cargo.lock", "apps/cli/Cargo.toml", "apps/mcp/Cargo.toml", "package.json", "package-lock.json", "packages/pixelart/package.json"]);
execFileSync("git", ["commit", "-m", `chore(release): ${version} [skip ci]`]);
execFileSync("git", ["tag", tag]);
execFileSync("git", ["push", "origin", `HEAD:${branch}`]);
execFileSync("git", ["push", "origin", tag]);

function makeNotes(items) {
  const sections = new Map([["feat", "Features"], ["fix", "Fixes"], ["perf", "Performance"], ["refactor", "Refactoring"]]);
  const grouped = new Map();
  for (const item of items) {
    const match = item.subject.match(/^([a-z]+)(?:\(([^)]*)\))?!?:\s*(.+)$/i);
    if (!match || !sections.has(match[1].toLowerCase())) continue;
    const [, type, scope, subject] = match;
    const heading = sections.get(type.toLowerCase());
    if (!grouped.has(heading)) grouped.set(heading, []);
    grouped.get(heading).push(`- ${scope ? `**${scope}:** ` : ""}${subject} (${item.hash.slice(0, 7)})`);
  }
  return [...grouped].map(([heading, entries]) => `## ${heading}\n\n${entries.join("\n")}`).join("\n\n") || "## Changes\n\nRelease includes breaking changes.";
}

function parseTag(raw) {
  const match = raw.match(/^v(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z.-]+))?$/);
  if (!match) return undefined;
  return { raw: raw.slice(1), major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]), prerelease: match[4]?.split(".") };
}

function compareVersions(a, b) {
  return a.major - b.major || a.minor - b.minor || a.patch - b.patch || comparePrerelease(a.prerelease, b.prerelease);
}

function comparePrerelease(a = [], b = []) {
  if (!a.length) return b.length ? 1 : 0;
  if (!b.length) return -1;
  for (let i = 0; i < Math.max(a.length, b.length); i++) {
    if (a[i] === undefined) return -1;
    if (b[i] === undefined) return 1;
    const left = /^\d+$/.test(a[i]) ? Number(a[i]) : a[i];
    const right = /^\d+$/.test(b[i]) ? Number(b[i]) : b[i];
    if (left === right) continue;
    if (typeof left === "number" && typeof right === "number") return left - right;
    if (typeof left === "number") return -1;
    if (typeof right === "number") return 1;
    return left < right ? -1 : 1;
  }
  return 0;
}
