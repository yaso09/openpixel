# Phase 20 — Agent skill installation

## Completed

- Added a short OpenPixel skill installation section to the English and Turkish READMEs.
- Extended Bash and PowerShell installers to install the skill globally through the Skills CLI when `npx` is available.
- Kept binary installation successful when Node.js/npm or skill installation is unavailable, and printed a retry command.

## Verification

- Confirmed `--global` and `--yes` are supported by the Skills CLI documentation.
- `git diff --check` — passed.
