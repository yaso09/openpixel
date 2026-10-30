# Phase 18 — Latest-release installers

## Completed

- Added Bash and PowerShell installers that fetch the latest GitHub Release assets and install both CLI and MCP binaries.
- Extended tag releases to build and attach Linux x86_64, macOS Apple Silicon, and Windows x64 packages.
- Added one-line install commands and platform support details to both READMEs.

## Verification

- `git diff --check` — passed.

## Platform coverage

- Bash installer supports Linux x86_64 and macOS Apple Silicon.
- PowerShell installer supports Windows x64 and registers the install directory in the user's PATH.
- Other operating systems and CPU architectures are not currently packaged.
