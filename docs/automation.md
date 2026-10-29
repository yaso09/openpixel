# GitHub automation

OpenPixel uses OpenCode GitHub workflows for maintainer-requested tasks, read-only pull request reviews, issue triage, and a weekly backlog report. Dependabot checks npm, Cargo, and GitHub Actions dependencies weekly. GitHub documents the `/` directory and weekly interval for Actions updates in its [Dependabot setup guide](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/auto-update-actions).

## Setup

1. Enable GitHub Actions for the repository.
2. Add an `OPENCODE_API_KEY` Actions secret using an OpenCode Go API key. The workflows use `opencode-go/gpt-5.6-luna`; see the [OpenCode Go guide](https://opencode.ai/docs/go/) for subscription and model details.
3. Review the permissions declared in each workflow. The mention-command workflow can create a branch or pull request when a trusted maintainer asks it to make a change. The review and triage workflows cannot edit files or run shell commands, and the scheduled report uses a read-only GitHub token.

The workflows use OpenCode's documented `use_github_token` mode. This passes each job's scoped `GITHUB_TOKEN` to the action, so a separate OpenCode GitHub App installation or OIDC permission is not required. The GitHub token permissions are declared per workflow. See the [OpenCode GitHub guide](https://opencode.ai/docs/github/) for the token mode and the [permissions guide](https://opencode.ai/docs/permissions/) for tool restrictions.

The `opencode.yml` command workflow listens to `/opencode` and `/oc` comments on issues and inline pull request review comments. It runs only when an owner, member, or collaborator uses a trigger. A maintainer can ask it to explain, review, or implement a request.

`opencode-review.yml` reviews non-draft pull requests opened from branches in this repository. It does not check out fork pull requests, edit files, run shell commands, approve, or merge.

`opencode-triage.yml` provides a concise summary on new issues opened by an owner, member, or collaborator. The weekly `opencode-maintenance.yml` report inspects open issues and pull requests and writes a report to the Actions run output. It also supports manual runs from the Actions tab.

## Workflow changes

- Keep third-party actions pinned to full commit SHAs.
- Preserve the trusted-author gate on comment-triggered and issue-triggered agents.
- Keep automatic review and triage read-only; maintainers decide labels, assignments, approvals, merges, and releases.
- Check schedules and model/API secret names when changing providers.
- Do not use `pull_request_target` to run OpenCode on contributor code.
