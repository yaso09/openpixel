# OpenPixel MCP server

The Rust MCP server exposes OpenPixel asset authoring, validation, SVG rendering, and MP4 video export as MCP tools. It shares the v1 document model, validation, and rendering pipeline with the CLI crate.

## Run locally

From the repository root:

```sh
cargo run -p openpixel-mcp -- stdio
cargo run -p openpixel-mcp -- http 127.0.0.1:8000
```

`stdio` is the default when no transport is supplied and is intended for MCP clients that launch a child process. The HTTP mode serves MCP at `/mcp`; the bind address is optional and defaults to `127.0.0.1:8000`. `OPENPIXEL_MCP_ADDR` can set the address when no address argument is provided.

## Connect an agent

After installing OpenPixel, run one of these commands to add the local stdio server to the agent's user configuration:

```sh
openpixel-mcp add codex
openpixel-mcp add claude
```

The Codex entry is added to `~/.codex/config.toml`. Claude Desktop uses `%APPDATA%\\Claude\\claude_desktop_config.json` on Windows and `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS. Existing server entries are preserved, and the config stores the full path to the installed executable. Restart the selected app after setup. Keep `openpixel-mcp` at that location; if you configure an agent manually, its command must resolve to the executable.

To configure another MCP-compatible agent manually, register a stdio server with command `openpixel-mcp` and argument `stdio`. For example, Claude Desktop uses:

```json
{
  "mcpServers": {
    "openpixel": {
      "command": "openpixel-mcp",
      "args": ["stdio"]
    }
  }
}
```

The HTTP mode uses the current MCP Streamable HTTP transport. Responses use Server-Sent Events (`text/event-stream`) when streaming is needed; the SDK does not implement the superseded 2024 two-endpoint HTTP+SSE transport. See the [official Rust SDK transport notes](https://github.com/modelcontextprotocol/rust-sdk#transports).

## Tools

- `version`, `update`, `create_asset`, `validate_asset`, `inspect_asset`
- `render_frame`, `render_all_frames`, `render_video`, `compare_assets`
- `set_pixel`, `set_layer`, `add_frame`, `set_frame_duration`, `set_loop`

`render_video` writes an H.264 MP4 to the requested `output_path`, follows frame durations, and accepts optional `fps` and pixel `scale` values. It requires ffmpeg with `libx264` available on `PATH`.

The `update` tool accepts optional `force: true` to reinstall the latest release when the MCP server is already on that version. Restart the server after updating.

All modifications take a source `path` and explicit `output_path`, so the caller decides whether to create a new file or update in place. New assets start at `revision: 1`; each successful edit tool increments the artwork revision, and `inspect_asset` reports it. Legacy assets without a revision are treated as revision 1. This is distinct from `version: 1`, which identifies the document schema. SVG rendering returns SVG content directly as tool text; video rendering writes the binary MP4 to the explicit destination. MCP tool failures return visible tool errors, while malformed protocol parameters are rejected through the MCP error channel.

Each successful edit tool opens a local compare window containing the previous and new revision. Use `compare_assets` to compare any two saved files manually. The window supports side-by-side, adjustable overlay, and pixel-diff views with a shared animation-frame selector. A headless environment may not be able to open the browser window; in that case the tool result includes the saved comparison page path.

The tool schemas and descriptions are generated from Rust parameter types and list through the MCP `tools/list` method.
