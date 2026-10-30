# Phase 22 — OpenCode release workflow push-event fix

## Completed

- Replaced `opencode github run` in the tag release workflow with the non-interactive OpenCode CLI, which works for the `push` event.
- Added a manual `workflow_dispatch` input so maintainers can retry a failed release for an existing tag.
- Applied a temporary OpenCode config that allows reading and editing release documentation while denying shell, subagent, and web tools.
- Updated release automation docs with the retry path.

## Verification

- Confirmed `opencode run` and `OPENCODE_CONFIG_CONTENT` usage against official OpenCode CLI and configuration documentation.
- `git diff --check` — passed.
