# Phase 22 — OpenCode release workflow push-event fix

> This release-note implementation was later replaced by Groq; OpenCode remains in the other repository workflows.

## Completed

- Replaced `opencode github run` in the tag release workflow with the non-interactive OpenCode CLI, which works for the `push` event.
- Added a manual `workflow_dispatch` input so maintainers can retry a failed release for an existing tag.
- Added a workflow step that collects commits since the previous release tag and gives that list to OpenCode as release-note input.
- Applied a temporary OpenCode config that allows reading and editing release documentation while denying shell, subagent, and web tools.
- Updated release automation docs with the retry path.

## Verification

- Confirmed `opencode run` and `OPENCODE_CONFIG_CONTENT` usage against official OpenCode CLI and configuration documentation.
- Confirmed the workflow computes the commits since the prior tag before asking OpenCode to draft the changelog and release notes.
- `git diff --check` — passed.
