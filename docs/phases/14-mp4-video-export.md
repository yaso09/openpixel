# Phase 14 — MP4 video export

## Completed

- Added a Rust MP4 export pipeline that composites OpenPixel layers and streams RGB frames to ffmpeg with H.264 encoding.
- Added the `openpixel render-video` CLI command with output path, frame rate, and pixel scale options.
- Added an MCP `render_video` tool with explicit MP4 output path and optional frame rate and pixel scale.
- Preserved per-frame `durationMs` by sampling the animation timeline at the chosen output frame rate.
- Documented the ffmpeg dependency and export options in English and Turkish project documentation.

## Limits

- Requires ffmpeg with `libx264` available on `PATH`.
- Output frame rate is 1–60 fps; video duration is capped at 10 minutes; scaled output is capped at 16,777,216 pixels per frame.
- Transparent pixels composite over black in MP4 output.

## Verification

- `cargo check -p openpixel-cli -p openpixel-mcp` — passed.
