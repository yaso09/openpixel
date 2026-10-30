# Phase 17 — OpenCode release notes and changelog

## Completed

- Updated the tag release workflow to ask OpenCode to create/update the root `CHANGELOG.md` and draft the release notes.
- Selected `opencode/deepseek-v4.1-flash-free`, currently listed as a free OpenCode model.
- The workflow commits the changelog update to the default branch and creates the GitHub Release with OpenCode's notes.
- Documented the release flow and model choice.

## Verification

- Confirmed the free model ID in the official [OpenCode Console model catalog](https://opencode.ai/v2/docs/console/models/).
- `git diff --check` — passed.

## Operational note

- Free model availability can change. If the selected model is removed, update the model ID from the official OpenCode catalog.
