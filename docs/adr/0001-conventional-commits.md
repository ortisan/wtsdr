# ADR 0001: Adopt Conventional Commits and enforce via tooling

- Status: Accepted
- Date: 2025-09-07

## Context
A consistent commit message format improves readability, enables automated changelogs, and supports semantic versioning. The project intends to automate releases and changelog generation, which benefits from Conventional Commits.

## Decision
Adopt the Conventional Commits specification for all commits in this repository.

Conventional Commits format:
- type(scope)?: subject
- Use lowercase type; optional scope in parentheses; subject should be concise and not end with a period.
- Include breaking changes either with a `!` after the type/scope (e.g., `feat!:`) or with a `BREAKING CHANGE:` footer.

Allowed types include (as configured): feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert.

## Enforcement
- Local (pre-commit-msg hook):
  - Husky + @commitlint enforce message format at commit time.
  - Configuration files: `commitlint.config.js` and `.husky/commit-msg`.
- Continuous Integration:
  - CI validates commit messages using `rusk`, a Rust-based Conventional Commits linter.
  - Workflow file: `.github/workflows/commitlint.yml`.
  - Rules configured in `rusk.toml` (allowed types, max subject length, no trailing period, breaking change handling, etc.).

## Consequences
- Clear, structured commit history that can be parsed for changelog and release notes.
- Enables automated release tooling (see ADR-0002 on Release Please) to infer version bumps:
  - feat → minor, fix → patch, BREAKING CHANGE → major.
- Contributors must learn and follow the Conventional Commits format; CI and hooks will block non-compliant messages.

## Alternatives considered
- Unstructured commit messages: rejected due to lack of automation and inconsistency.
- Custom in-house guidelines without tooling: rejected because enforcement would be manual and error-prone.

## References
- Local lint config: `commitlint.config.js`
- Husky hook: `.husky/commit-msg`
- CI lint workflow: `.github/workflows/commitlint.yml`
- Rusk config: `rusk.toml`
- Conventional Commits spec: https://www.conventionalcommits.org/
- Related decision: ADR-0002 (Release Please)
