# OpenPixel MCP server

The Rust MCP server exposes OpenPixel asset authoring, validation, SVG rendering, and MP4 video export as MCP tools. It shares the v1 document model, validation, and rendering pipeline with the CLI crate.

## Run locally

From the repository root:

```sh
cargo run -p openpixel-mcp -- stdio
cargo run -p openpixel-mcp -- http 127.0.0.1:8000
```

`stdio` is the default when no transport is supplied and is intended for MCP clients that launch a child process. The HTTP mode serves MCP at `/mcp`; the bind address is optional and defaults to `127.0.0.1:8000`. `OPENPIXEL_MCP_ADDR` can set the address when no address argument is provided.

The HTTP mode uses the current MCP Streamable HTTP transport. Responses use Server-Sent Events (`text/event-stream`) when streaming is needed; the SDK does not implement the superseded 2024 two-endpoint HTTP+SSE transport. See the [official Rust SDK transport notes](https://github.com/modelcontextprotocol/rust-sdk#transports).

## Tools

- `create_asset`, `validate_asset`, `inspect_asset`
- `render_frame`, `render_all_frames`, `render_video`
- `set_pixel`, `set_layer`, `add_frame`, `set_frame_duration`, `set_loop`

`render_video` writes an H.264 MP4 to the requested `output_path`, follows frame durations, and accepts optional `fps` and pixel `scale` values. It requires ffmpeg with `libx264` available on `PATH`.

All modifications take a source `path` and explicit `output_path`, so the caller decides whether to create a new file or update in place. SVG rendering returns SVG content directly as tool text; video rendering writes the binary MP4 to the explicit destination. MCP tool failures return visible tool errors, while malformed protocol parameters are rejected through the MCP error channel.

The tool schemas and descriptions are generated from Rust parameter types and list through the MCP `tools/list` method.
