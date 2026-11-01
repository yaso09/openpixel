<div align="center">
  <img src="banner.svg" alt="OpenPixel pixel art landscape" width="100%" />

  # OpenPixel

  **Create pixel art with agents. Render it in the browser, export SVGs, or turn animations into MP4.**

  [![TypeScript](https://img.shields.io/badge/renderer-TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white)](packages/pixelart)
  [![Rust](https://img.shields.io/badge/tools-Rust-000000?style=flat-square&logo=rust&logoColor=white)](apps)
  [![MIT](https://img.shields.io/badge/license-MIT-8A5CF6?style=flat-square)](LICENSE)

  [Install](#install) · [Quick start](#quick-start) · [Renderer](#typescript-renderer) · [CLI](#cli) · [MCP](#mcp-server) · [Development](#development)
</div>

OpenPixel is a small toolkit for creating and using pixel art images and animations. An asset is a JSON document built from ordered layers of pixel matrices and animation frames. AI agents can author and edit the same format through the CLI or MCP server; the TypeScript library renders it as crisp SVG in a browser.

## What you can do

- Draw and animate layered pixel art in a shared, validated JSON format.
- Render a still frame or play an animation as SVG in the browser.
- Create, inspect, validate, and edit assets from the command line or an MCP client.
- Export each animation frame as SVG or encode timed frames as MP4.
- Give an AI agent structured tools for creating and refining artwork.
- Follow the [OpenPixel pixel art skill](skills/openpixel/SKILL.md) for agent drawing workflows and quality checks.

## Install

### CLI and MCP server

Install the latest GitHub Release on Linux x86_64 or Apple Silicon macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/yaso09/openpixel/master/install.sh | bash
```

On Windows x64, run in PowerShell:

```powershell
irm https://raw.githubusercontent.com/yaso09/openpixel/master/install.ps1 | iex
```

The installers add `openpixel` and `openpixel-mcp`. Bash installs to `~/.local/bin` by default; set `OPENPIXEL_INSTALL_DIR` to choose another location. PowerShell installs to `%LOCALAPPDATA%\OpenPixel\bin` and adds that directory to your user PATH. Open a new terminal after installing on Windows.

### OpenPixel agent skill

Install the pixel art skill for your coding agent with the [Skills CLI](https://skills.sh/):

```bash
npx skills add https://github.com/yaso09/openpixel/skills --skill openpixel-pixel-art
```

The platform installers also install this skill globally when Node.js/npm is available.

### TypeScript renderer

```bash
npm i -g github:yaso09/openpixel
```

> [!NOTE]
> The TypeScript renderer is distributed from this GitHub repository and will not be published to npm.

## Quick start

Create a layered asset, paint a pixel, then render it to SVG:

```bash
openpixel new art/creature.json --width 16 --height 16 --name Creature --layers body:Body,details:Details
openpixel set-pixel art/creature.json --output art/creature-painted.json --frame 0 --layer body --x 7 --y 6 --color '#E96B55'
openpixel validate art/creature-painted.json
openpixel render art/creature-painted.json --scale 8 --output out/creature.svg
```

Edits write to an explicit output path so the source asset stays intact. Coordinates and frame indices start at zero. See [the animated example](examples/two-frame-spark.json) for a complete multi-frame document.

## TypeScript renderer

Render a document directly to SVG:

```ts
import { renderToSvg, type PixelArtDocument } from "@openpixel/renderer";

const art: PixelArtDocument = {
  version: 1,
  revision: 1,
  name: "One pixel",
  width: 1,
  height: 1,
  layers: [{ id: "ink", name: "Ink" }],
  frames: [{
    durationMs: 100,
    layers: [{ layerId: "ink", pixels: [["#F04438"]] }]
  }]
};

document.querySelector("#art")!.innerHTML = renderToSvg(art, {
  pixelSize: 16,
  title: "A red pixel"
});
```

Use `mountPixelArt(element, art)` for browser playback. The player exposes `play()`, `pause()`, `seek(frameIndex)`, and `dispose()`. See the [renderer guide](packages/pixelart/README.md) and [v1 JSON schema](schemas/pixel-art.v1.schema.json).

## CLI

`openpixel` provides commands for asset creation and editing, inspection, validation, revision comparison, SVG frame rendering, and video export. Run `openpixel help` for all arguments.

```bash
openpixel inspect art/creature.json
openpixel render-frames art/creature.json --scale 4 --out-dir out/frames
openpixel render-video art/creature.json --output out/creature.mp4 --fps 30 --scale 4
openpixel compare art/creature-v1.json art/creature-v2.json
```

Run `openpixel update` to install a newer release, or `openpixel update --force` to reinstall the latest release even when the current version is already installed.

MP4 export requires `ffmpeg` with `libx264` available on `PATH`. It follows each frame's `durationMs`; the frame rate can be set from 1 to 60 fps.

## MCP server

The Rust MCP server supports stdio for local MCP clients and Streamable HTTP for remote connections. HTTP responses use Server-Sent Events when streaming is needed.

```bash
# stdio transport (default)
openpixel-mcp stdio

# HTTP transport, listening on localhost:8000
openpixel-mcp http 127.0.0.1:8000
```

Available tools include `version`, `update`, `create_asset`, `validate_asset`, `inspect_asset`, `compare_assets`, `render_frame`, `render_all_frames`, `render_video`, `set_pixel`, `set_layer`, `add_frame`, `set_frame_duration`, and `set_loop`. Set `force: true` on the MCP `update` tool to reinstall the latest release when already current. See [MCP setup and tool details](apps/mcp/README.md).

### Connect Codex or Claude Desktop

After installing OpenPixel, run the matching command to register the local MCP server automatically, then restart the app:

```bash
openpixel-mcp add codex
openpixel-mcp add claude
```

For another MCP-compatible agent, add a stdio server with command `openpixel-mcp` and argument `stdio`. See the [MCP setup guide](apps/mcp/README.md) for a manual configuration example and config file locations.

## Asset format

OpenPixel documents contain a schema `version`, an artwork `revision`, canvas size, ordered layer definitions, and one or more frames. The schema version remains `1`; each newly created artwork starts at revision `1`, and successful CLI/MCP edits increment its revision and open a local comparison window. The window offers side-by-side, adjustable overlay, and pixel-diff views, plus frame selection for animations. Legacy files without `revision` are treated as revision `1`. CLI `inspect` and MCP `inspect_asset` display the current revision. Each frame stores a two-dimensional color matrix for its layers; `null` represents a transparent pixel. Frame durations drive browser playback and video export. The shared [v1 schema](schemas/pixel-art.v1.schema.json) defines the format used by the renderer, CLI, and MCP server.

## Development

Requirements: Node.js 20 or later and a stable Rust toolchain.

```bash
npm install
npm run build
cargo build --release --workspace
```

The monorepo contains the browser renderer in `packages/pixelart`, the CLI in `apps/cli`, the MCP server in `apps/mcp`, and shared schemas in `schemas/`. For contribution guidance, see [CONTRIBUTING.md](CONTRIBUTING.md). GitHub Actions creates a release and platform packages for each pushed tag; see [automation details](docs/automation.md).

## Versioning and releases

Releases use Semantic Versioning and Conventional Commits. On `master`, `feat:` creates a minor release, `fix:`/`perf:`/`refactor:` create a patch release, and a `BREAKING CHANGE:` footer or `!` suffix creates a major release. `docs:`, `style:`, `test:`, `build:`, `ci:`, and `chore:` commits do not release. Use `npm run release:dry` to preview the next version and notes locally. Pushes on `alpha` and `beta` create prereleases. Each release synchronizes the npm and Cargo manifests, updates `CHANGELOG.md`, tags `vMAJOR.MINOR.PATCH` (with `-alpha.N` or `-beta.N` on prerelease branches), and starts the platform binary packaging workflow.

## Automation

OpenCode scans the repository daily for verified defects. It can create up to two issues and open a focused fix PR for each issue it creates. PRs are not merged automatically. See [GitHub automation details](docs/automation.md).

For each release tag, Groq's Free plan runs `openai/gpt-oss-120b` to generate English release notes and update `CHANGELOG.md` using application and package changes only. Documentation, workflow, and other repository maintenance commits are excluded. The same notes appear in the GitHub Release. Keep the Groq account on the Free plan to avoid usage charges.
