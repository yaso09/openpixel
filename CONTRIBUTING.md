# Contributing to OpenPixel

Thanks for helping improve OpenPixel. Human and AI-assisted contributions are welcome. Contributors are responsible for reviewing and understanding the changes they submit, regardless of how the changes were produced.

## Before starting

- Read the [roadmap](docs/ROADMAP.md) and the relevant [phase report](docs/phases/).
- For substantial changes, open or comment on an issue first so the intended behavior is clear.
- Keep changes focused and use the existing JSON v1 contract unless a format change is part of the proposal.

## Development setup

Requirements: Node.js 20 or newer, npm, Rust stable with Cargo, and Git.

```sh
npm install
npm run build
cargo fmt --all -- --check
cargo check --workspace --offline
```

The renderer lives in `packages/pixelart`, the CLI in `apps/cli`, the MCP server in `apps/mcp`, and the shared JSON Schema in `schemas/`. The MCP server and CLI use shared Rust model/validation/rendering code in the CLI crate.

## Making changes

- Update the relevant documentation and phase report when completing a planned phase or changing an interface.
- Keep JSON Schema, TypeScript validation, and Rust validation behavior aligned.
- Preserve transparent pixels, layer ordering, frame timing, and renderer output semantics.
- Do not commit generated build output, credentials, API keys, or private asset files.
- For GitHub automation changes, pin third-party actions to full commit SHAs and keep the pins updated through Dependabot.

## Verification

Before submitting a pull request, run the checks that match your change:

- TypeScript renderer or npm package: `npm run build` and `npm pack --dry-run --workspace @openpixel/renderer`.
- Rust CLI or MCP: `cargo fmt --all -- --check` and `cargo check --workspace --offline`.
- JSON format or examples: parse the JSON and verify it against `schemas/pixel-art.v1.schema.json`.

Include any checks you ran and call out checks you could not run. Add automated tests for behavior changes when practical; do not rely only on manual verification.

## Commit messages

Use English Conventional Commit messages, for example:

- `feat(renderer): add frame export options`
- `fix(cli): reject invalid pixel colors`
- `docs: clarify MCP transport setup`
- `chore: update GitHub action pins`

## Pull requests

A pull request should explain the problem, the approach, compatibility impact, and verification. Link related issues when available. Keep each change reviewable and respond to feedback with follow-up commits or updates.

## AI-assisted contributions

AI tools may help with implementation, documentation, and review. The submitting contributor must:

- Read and verify AI-generated changes, including their dependencies and generated files.
- Check that examples and claims match the implementation.
- Disclose significant AI assistance when the repository host or contribution policy asks for it.
- Never include secrets, user data, or private repository contents in prompts or commits.

OpenCode automation is a helper. Daily scans may create issues and focused fix PRs for verified defects; automatic PR review and issue triage remain read-only. Maintainers make final decisions about labels, approval, merge, and release. See [GitHub automation setup](docs/automation.md) for tokens and workflow permissions.
