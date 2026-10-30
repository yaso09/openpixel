# Phase 21 — Daily issue and fix PR scan

## Completed

- Changed the OpenCode maintenance workflow from a weekly read-only report to a daily repository scan.
- Granted that workflow scoped `contents`, `issues`, and `pull-requests` write permissions.
- Directed OpenCode to check for duplicates, create issues for verified defects, and open focused, unmerged fix PRs for issues created by the same run.
- Capped the run at two verified issues and updated Turkish README, contribution, and automation documentation.

## Verification

- `git diff --check` — passed.

## Operational note

- The workflow uses the repository's `OPENCODE_API_KEY` secret and runs daily at 09:20 UTC.
