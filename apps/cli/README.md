# OpenPixel CLI

`openpixel` creates and edits OpenPixel v1 JSON assets, validates them, renders SVG output, and encodes animations as MP4. Build from the repository root with `cargo build --release -p openpixel-cli`; the executable is `target/release/openpixel` (or `openpixel.exe` on Windows).

Video export requires `ffmpeg` with the `libx264` encoder available on `PATH`.

## Examples

```sh
openpixel new assets/hero.json --width 16 --height 16 --name Hero --layers body:Body,details:Details
openpixel set-pixel assets/hero.json --output assets/hero-painted.json --frame 0 --layer body --x 7 --y 6 --color '#E96B55'
openpixel add-frame assets/hero-painted.json --output assets/hero-walk.json --duration-ms 120
openpixel set-frame assets/hero-walk.json --output assets/hero-walk.json --frame 0 --duration-ms 150
openpixel set-layer assets/hero-walk.json --output assets/hero-ready.json --layer details --visible false --opacity 0.5
openpixel validate assets/hero-ready.json
openpixel render assets/hero-ready.json --frame 1 --scale 8 --title 'Hero, frame 2' --output out/hero.svg
openpixel render-frames assets/hero-ready.json --scale 4 --out-dir out/hero-frames
openpixel render-video assets/hero-walk.json --output out/hero.mp4 --fps 30 --scale 4
openpixel update --force
```

`openpixel update` installs the latest release when a newer version is available. Add `--force` to reinstall the latest release even when the installed version is already current.

Commands write to an explicit output path so edits do not silently overwrite the source. `new` creates transparent matrices; `set-pixel` accepts `null` to erase a pixel. Frame indices and coordinates are zero-based. SVG exports preserve layer order, visibility, opacity, transparent pixels, and sharp pixel edges. MP4 export follows each frame's `durationMs`; `--fps` sets the output frame rate (1–60, default 30), and `--scale` sets integer pixel enlargement (1–16, default 1).

## Feature coverage

| Renderer capability | CLI access |
|---|---|
| Validate v1 shape, dimensions, colors, layer references | `validate` (and automatically before all operations) |
| Render any still frame | `render --frame` |
| Frame size / crisp SVG output | `render --scale` |
| Background and accessible title | `render --background --title` |
| Layer visibility, opacity, and ordering | stored asset fields; `set-layer` edits visibility/opacity/name |
| Animation frames and durations | `inspect`, `add-frame`, `set-frame`, `render-frames` |
| MP4 video export with frame timing | `render-video` (requires ffmpeg with libx264) |
| Looping | `inspect`, `set-loop` |
| Browser playback controls and per-frame callback | Browser-only API in the TypeScript package; CLI exports ordered frames and timed video |

The CLI and browser package consume the same JSON contract. The browser player itself requires a DOM; the CLI exposes visual output through SVG and video export.
