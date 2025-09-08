# ADR 0002: Use Release Please for automated releases and changelog

- Status: Accepted
- Date: 2025-09-07

## Context
We want to automate versioning, tagging, GitHub Releases, and CHANGELOG generation based on commit history. Manual releases are error-prone and time-consuming. Since we already enforce Conventional Commits (see ADR-0001), we can leverage commit messages to drive semantic versioning and changelog sections.

## Decision
Adopt Google’s release-please GitHub Action to manage releases.

Configuration (as of this ADR):
- Workflow: `.github/workflows/release.yml`
- Trigger: `push` to the `main` branch
- Permissions: `contents: write`, `pull-requests: write`
- Action: `google-github-actions/release-please-action@v4`
- Release type: `simple`
- Package name: `wtsdr`
- Auth: `${{ secrets.RELEASEME }}` (a PAT stored as a GitHub secret)
- Changelog mapping: custom `changelog-types` mapping sections for feat, fix, perf, revert, docs, style, refactor, test, build, ci (some hidden)

Operational flow:
1. Conventional Commits are merged into `main`.
2. The action analyzes commits and opens/updates a Release PR or cuts a release per release-please logic.
3. When the Release PR is merged, release-please creates a tag, a GitHub Release, and updates `CHANGELOG.md` with categorized entries.

## Consequences
- Automated and consistent releases and changelogs tied to commit history.
- Version bumps follow Conventional Commit semantics (feat → minor, fix → patch, BREAKING CHANGE → major).
- Requires maintaining the `RELEASEME` secret with appropriate repository permissions.
- Release notes structure follows the configured changelog mapping; developers should use approved Conventional Commit types.

## Alternatives considered
- Manual release process: rejected due to maintenance burden and inconsistency.
- Other release automation tools (e.g., semantic-release): viable but we prefer release-please’s PR-first model and native GitHub Action.

## References
- Workflow: `.github/workflows/release.yml`
- README: “Release automation (Release Please)” section
- Release Please docs: https://github.com/googleapis/release-please
- Related decision: ADR-0001 (Conventional Commits)
