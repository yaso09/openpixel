# Phase 15 — Automatic releases from tags

## Completed

- Added a GitHub Actions workflow triggered by every pushed tag.
- The workflow verifies the tag and creates a GitHub Release named after it, with GitHub-generated release notes.
- Limited the workflow token to `contents: write` and documented how maintainers publish a release.
- Updated the English and Turkish README automation summaries.

## Verification

- Reviewed the workflow trigger, token permissions, and `gh release create` arguments.
- `git diff --check` — passed.

## Next

- Push a tag to the GitHub repository to verify the first hosted release run.
