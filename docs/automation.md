# GitHub automation

OpenPixel uses OpenCode GitHub workflows for maintainer-requested tasks, read-only pull request reviews, issue triage, a daily repository scan that can open issues and fix PRs, and release documentation. Pushing any Git tag builds CLI/MCP packages for Linux x86_64, macOS Apple Silicon, and Windows x64, then runs OpenCode with the free `opencode/deepseek-v4.1-flash-free` model to update `CHANGELOG.md` and draft release notes. The workflow commits the changelog update to the default branch and attaches the platform packages to the GitHub Release. Dependabot checks npm, Cargo, and GitHub Actions dependencies weekly. GitHub documents the `/` directory and weekly interval for Actions updates in its [Dependabot setup guide](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/auto-update-actions).

## Setup

1. Enable GitHub Actions for the repository.
2. Add an `OPENCODE_API_KEY` Actions secret using an OpenCode Zen API key. The workflows use `opencode/gpt-5.6-luna`; see the [OpenCode Zen guide](https://opencode.ai/docs/zen/) for setup and available models.
3. Review the permissions declared in each workflow. The mention-command workflow can create a branch or pull request when a trusted maintainer asks it to make a change. The daily scan can create up to two issues and corresponding fix pull requests per run. Automatic PR review and issue triage remain read-only with respect to source code.

The workflows use OpenCode's documented `use_github_token` mode. This passes each job's scoped `GITHUB_TOKEN` to the action, so a separate OpenCode GitHub App installation or OIDC permission is not required. The GitHub token permissions are declared per workflow. See the [OpenCode GitHub guide](https://opencode.ai/docs/github/) for the token mode and the [permissions guide](https://opencode.ai/docs/permissions/) for tool restrictions.

The `opencode.yml` command workflow listens to `/opencode` and `/oc` comments on issues and inline pull request review comments. It runs only when an owner, member, or collaborator uses a trigger. A maintainer can ask it to explain, review, or implement a request.

`opencode-review.yml` reviews non-draft pull requests opened from branches in this repository. It does not check out fork pull requests, edit files, run shell commands, approve, or merge.

`opencode-triage.yml` provides a concise summary on new issues opened by an owner, member, or collaborator. The `opencode-maintenance.yml` workflow runs daily (and can be started manually), scans for verified defects, checks existing issues and pull requests for duplicates, creates up to two issues, then creates a separate fix PR for each issue it created. It does not merge PRs or close/assign issues.

`release-on-tag.yml` runs for every pushed tag. It builds CLI and MCP binaries for supported platforms, collects commits since the preceding release tag, then runs the OpenCode CLI to update the root `CHANGELOG.md` and write `release-notes.md` from that commit list. The workflow commits the changelog update to the repository's default branch and creates a GitHub Release named after the tag using the generated notes and binary packages. It uses the `OPENCODE_API_KEY` Actions secret and the repository-scoped `contents: write` permission. Create and push a tag with `git tag vX.Y.Z` followed by `git push origin vX.Y.Z` to publish a release.

If a tag run fails, the same workflow can be started from the Actions tab using **Run workflow** and the existing tag name (for example, `v0.1.0`). This runs the supported `workflow_dispatch` event and republishes that tag.

## Workflow changes

- Keep third-party actions pinned to full commit SHAs.
- Preserve the trusted-author gate on comment-triggered and issue-triggered agents.
- Keep automatic PR review and issue triage read-only; the daily scan may create issues and fix PRs, but never merges or closes them. Releases are created automatically when a tag is pushed.
- Check schedules and model/API secret names when changing providers.
- Do not use `pull_request_target` to run OpenCode on contributor code.
