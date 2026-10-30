# OpenPixel

OpenPixel is a toolkit for creating, editing, and rendering pixel art images and animations. Assets are JSON documents made from ordered pixel-matrix layers and frames. The TypeScript package renders those assets as crisp SVG in browsers; the Rust CLI and MCP server create and edit compatible assets.

## Packages

- [`@openpixel/renderer`](packages/pixelart/README.md): TypeScript types, validation, SVG frame rendering, and browser animation player.
- [`openpixel`](apps/cli/README.md): Rust command-line asset authoring, SVG export, and MP4 video encoding.
- [`openpixel-mcp`](apps/mcp/README.md): Rust MCP server over stdio and Streamable HTTP/SSE, including MP4 video export.
- [OpenPixel Pixel Art skill](.agents/skills/openpixel-pixel-art/SKILL.md): source-backed drawing and animation guidance for AI agents using OpenPixel.

## Install the renderer

After publishing, install it in an application with:

```sh
npm install @openpixel/renderer
```

## Install the CLI and MCP server

On Linux x86_64 or macOS Apple Silicon:

```sh
curl -fsSL https://raw.githubusercontent.com/yaso09/openpixel/main/install.sh | bash
```

On Windows x64 PowerShell:

```powershell
irm https://raw.githubusercontent.com/yaso09/openpixel/main/install.ps1 | iex
```

Both installers fetch the latest GitHub Release and install `openpixel` and `openpixel-mcp`. The Bash installer uses `~/.local/bin` by default; set `OPENPIXEL_INSTALL_DIR` to choose another directory. The PowerShell installer installs under `%LOCALAPPDATA%\OpenPixel\bin` and adds it to the user PATH.

For repository development, use npm workspaces:

```sh
npm install
npm run build
```

The Rust applications build independently from the repository root with Cargo:

```sh
cargo build --release --workspace
```

## Render an SVG frame

```ts
import { renderToSvg, type PixelArtDocument } from "@openpixel/renderer";

const asset: PixelArtDocument = {
  version: 1,
  name: "One pixel",
  width: 1,
  height: 1,
  layers: [{ id: "ink", name: "Ink" }],
  frames: [{
    durationMs: 100,
    layers: [{ layerId: "ink", pixels: [["#F04438"]] }]
  }]
};

document.querySelector("#art")!.innerHTML = renderToSvg(asset, {
  pixelSize: 16,
  title: "A red pixel"
});
```

`mountPixelArt(element, asset)` adds browser playback with `play()`, `pause()`, `seek(frameIndex)`, and `dispose()`. The renderer API and JSON format are described in [packages/pixelart/README.md](packages/pixelart/README.md) and [the v1 schema](schemas/pixel-art.v1.schema.json).

## Create an asset with the CLI

```sh
openpixel new art/creature.json --width 16 --height 16 --name Creature --layers body:Body,details:Details
openpixel set-pixel art/creature.json --output art/creature-painted.json --frame 0 --layer body --x 7 --y 6 --color '#E96B55'
openpixel render art/creature-painted.json --scale 8 --output out/creature.svg
```

## Project status

OpenPixel is in active development. See the [roadmap](docs/ROADMAP.md) and [phase reports](docs/phases/).

## GitHub automation

OpenCode handles maintainer commands, read-only issue/PR triage, weekly maintenance reports, and release notes. Pushing a Git tag updates `CHANGELOG.md` and creates a GitHub Release. Dependabot checks npm, Cargo, and GitHub Actions dependencies. See [automation setup](docs/automation.md).

## Contributing

People and AI-assisted contributions are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before making changes.

## License

MIT. See [LICENSE](LICENSE).
