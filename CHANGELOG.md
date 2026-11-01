# Changelog

This file is automatically updated for each release.

---

## [v0.4.0] - 2026-11-01

## Added
- Open a revision comparison window after edits with side-by-side, overlay, and pixel-diff views for stills and animation frames.
- Add `openpixel compare` and MCP `compare_assets` to compare saved artwork revisions.

---

## [v0.3.0] - 2026-11-01

## Added
- Track artwork edit revisions across CLI and MCP operations, starting new assets at revision 1 and reporting revisions during inspection.

---

## [v0.2.0] - 2026-10-31

## Added
- Add `--force` to CLI and MCP update commands so users can reinstall the current version.

## Changed
- Use the `self_update` Rust dependency for binary updates.

---

## [v0.1.3] - 2026-10-30

## Added
- Add `openpixel-mcp add codex` and `openpixel-mcp add claude` to register OpenPixel with desktop agents.
- Document MCP setup for Codex, Claude Desktop, and other compatible agents.

---


## [v0.1.2] - 2026-10-30

## Fixed
- Updated installer shell scripts to use raw strings, fixing quoting issues during updates.  
- The updater now correctly skips the update process when the application is already at the latest version.

---

## [v0.1.1] - 2026-10-30

## Added
- New `update` command
- New `version` command

---

## [v0.1.0] - 2026-10-30

## Added
- Export OpenPixel animations as MP4 files.  
- New detailed 4096 × 2304 pixel landscape.  
- Native drawing support for 2048 × 1152 pixel landscapes.  
- Animated OpenPixel landscape on the website.  
- MCP server now includes stdio and Streamable HTTP interfaces.  
- CLI now supports OpenPixel asset authoring and SVG export.  
- Renderer defines a pixel asset format and provides an SVG player.

## Changed
- Landing page simplified to highlight the pixel landscape.
