# What Is There Project

This repository uses automated releases and commit message linting to keep a clean history and CHANGELOG.

## Running

1) Generate secrets (creates/updates .env in project root):

```sh
chmod +x scripts/generate-secrets.sh
scripts/generate-secrets.sh
```

2) Start services:

```sh
docker compose up -d
```

Notes:
- docker-compose.yaml reads secrets from .env (POSTGRES_PASSWORD, AUTHELIA_* and BACKEND_AUTH_SECRET).
- Authelia admin password is set via AUTHELIA_ADMIN_PASSWORD and used in users_database.yaml.
- Postgres will run config/database/init.sql automatically on first initialization of the volume. To re-run, remove the postgres-data volume: docker compose down -v
- Postgres metrics exporter is available at http://localhost:9187/metrics (service: postgres-exporter). Point Prometheus to scrape this endpoint.


## Release automation (Release Please)

We use Google’s release-please via GitHub Actions to manage versioning, tags, GitHub Releases, and the CHANGELOG.

- Workflow: .github/workflows/release.yml
- Trigger: push events to the main branch
- Release type: simple (monorepo features are not used)
- Changelog sections: Features, Bug Fixes, Performance Improvements, Reverts, Documentation, Code Refactoring, etc. (see workflow for full mapping)
- Auth: uses a personal access token stored as the RELEASEME secret

How it works
1) You merge Conventional Commit(s) into main.
2) The workflow analyzes commit messages and opens/updates a Release PR or cuts a release if a PR is already ready according to release-please logic.
3) When the Release PR is merged, release-please creates:
   - a new Git tag
   - a GitHub Release
   - a CHANGELOG.md update with categorized entries

Initial setup (one-time)
- Create a GitHub secret named RELEASEME with a PAT that has repo permissions (a regular GITHUB_TOKEN often works, but this workflow is configured to read from RELEASEME explicitly).
- Ensure the default branch is main (as configured in the workflow).

Notes
- The versioning is inferred from Conventional Commits (feat -> minor, fix -> patch, BREAKING CHANGE -> major).
- You can customize behavior by editing .github/workflows/release.yml.

## Conventional Commit linting

This project enforces Conventional Commits both locally (via Husky+Commitlint) and in CI (via rusk).

### Local linting (Husky + @commitlint)

Already configured in this repo:
- commitlint.config.js with @commitlint/config-conventional
- Husky commit-msg hook at .husky/commit-msg
- package.json with devDependencies and a prepare script to install Husky

Getting started
1) Install dependencies: npm install
2) Enable Husky hooks (after install): npm run prepare
3) Make a commit. The commit-msg hook will run commitlint and reject invalid messages.

Manual checks
- Lint the last 5 commits: npx commitlint --from HEAD~5 --to HEAD
- Lint an edited message file: npx commitlint --edit .git/COMMIT_EDITMSG

Conventional Commit examples
- Good: feat(api): add create user endpoint
- Good: fix: prevent panic on empty payload
- Good (breaking): feat!: drop deprecated v1 endpoints
- Good (breaking footer): feat(api): migrate auth provider\n\nBREAKING CHANGE: tokens are no longer supported
- Avoid: added new feature, update stuff, misc changes

### CI linting (rusk)

In addition to local hooks, CI validates commits using rusk (a Rust-based Conventional Commits linter).

- Workflow: .github/workflows/commitlint.yml
- What it does: On PRs and pushes to main, runs rusk against the relevant commit range.
- Config: rusk.toml at the repo root controls the rules (allowed types, subject length, breaking change rules, etc.).

Optional local usage
- Install: cargo install rusk
- Validate your last commit: rusk --config rusk.toml last 1
- Validate a range: rusk --config rusk.toml range <from>..<to>

## Troubleshooting
- Husky not running: make sure npm run prepare was executed and .husky/commit-msg is executable (git preserves exec bits). Re-run: npx husky init (or npm run prepare).
- Releases not created: confirm the RELEASEME secret exists and has repo scope; check Action logs in the Release and Changelog workflow.
- Commits rejected: follow Conventional Commits format: type(scope)?: subject, with lowercase type and no trailing period; include BREAKING CHANGE when needed.

