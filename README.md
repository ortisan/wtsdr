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

### Generate TLS Certs


openssl req -x509 -newkey rsa:4096 -keyout auth-server-key.pem -out auth-server-cert.pem -sha256 -days 3650 -nodes -subj "/C=BR/ST=SP/L=SP/O=ORTISAN/OU=CompanySectionName/CN=localhost"

## Configure Authelia variables (.env)

Authelia in this repo is configured via environment variables read from .env and injected into the Authelia container by docker-compose. The Authelia config file config/authelia/configuration.yaml references those variables.

Where values are used:
- docker-compose.yaml → service authelia → environment: AUTHELIA_* variables
- config/authelia/configuration.yaml → uses ${AUTHELIA_JWT_SECRET}, ${AUTHELIA_SESSION_SECRET}, ${AUTHELIA_STORAGE_ENCRYPTION_KEY}
- config/authelia/users_database.yaml → stores hashed user passwords (not read from .env)

Required variables
- AUTHELIA_JWT_SECRET: Used to sign Authelia JWTs. Aim for 32–64 bytes of high-entropy random data.
- AUTHELIA_SESSION_SECRET: Used to encrypt session cookies. Aim for 32–64 bytes.
- AUTHELIA_STORAGE_ENCRYPTION_KEY: Used to encrypt storage at rest (SQLite in this demo). Aim for 32–64 bytes.

Optional but relevant
- AUTHELIA_ADMIN_PASSWORD: A convenience plaintext you can use to generate a password hash for the admin user in users_database.yaml. Note: Authelia itself does not read this variable; you must convert it to a hash and place it in users_database.yaml.

Quick start (recommended)
- Run the helper to create .env with strong random values:
  - chmod +x scripts/generate-secrets.sh
  - scripts/generate-secrets.sh
- Then start services: docker compose up -d

Manual configuration
Create or edit .env and set the variables. Example:

AUTHELIA_JWT_SECRET=RANDOM_64_BYTE_BASE64_VALUE
AUTHELIA_SESSION_SECRET=RANDOM_64_BYTE_BASE64_VALUE
AUTHELIA_STORAGE_ENCRYPTION_KEY=RANDOM_64_BYTE_BASE64_VALUE

You can generate secure values with:
- Linux/macOS: openssl rand -base64 64 | tr -d '\n'

Setting the admin password (users_database.yaml)
- Authelia requires hashed passwords in config/authelia/users_database.yaml.
- To generate a secure argon2id hash for your chosen password:
  - docker run --rm -it authelia/authelia:latest \
    authelia crypto hash generate argon2 --password 'YOUR_PASSWORD'
- Copy the resulting hash and replace the password field for the admin user in config/authelia/users_database.yaml.

Cookie domain and redirect URL
- configuration.yaml sets:
  - session.domain: localhost (change this to your actual domain if exposing beyond localhost)
  - default_redirection_url: http://localhost:3000 (change to your frontend URL if different)

Rotation guidance
- To rotate any AUTHELIA_* secret:
  1) Update the value in .env (or re-run scripts/generate-secrets.sh and manually replace the ones you want to rotate).
  2) Restart Authelia: docker compose up -d authelia
  3) Users may need to sign in again after rotation (session invalidation).
- To rotate an admin/user password, generate a new hash and update users_database.yaml, then restart Authelia.

Ports and health
- Authelia listens on 9091 (mapped to localhost:9091). Health endpoint: http://localhost:9091/api/health

Troubleshooting
- 401/redirect loops: Ensure session.domain matches the domain you use in the browser and that time on host is correct.
- Config changes not applied: Verify your bind mounts in docker-compose.yaml point to config/authelia/*.yaml and restart the service.

## Passing environment variables from .env to containers (Docker Compose)

There are two related concepts when working with environment variables and Compose:
- Variable substitution in the compose file: Compose automatically loads a .env file in the project root to substitute ${VAR} placeholders in docker-compose.yaml.
- Passing environment variables into the container: Use env_file and/or environment to set variables inside the running container.

Quick patterns you can use

1) Easiest: pass all .env entries into the container
services:
  your-service:
    image: your/image
    env_file:
      - .env

This will inject all key=value pairs from .env into the container process environment. Use this when the container expects these variables by name.

2) Mix and match: pass all, but also set explicit ones (with defaults)
services:
  your-service:
    image: your/image
    env_file:
      - .env
    environment:
      - DB_HOST=${POSTGRES_HOST}
      - DB_PORT=${POSTGRES_PORT:-5432}  # default if not set in .env
      - DB_USER=${POSTGRES_USER}
      - DB_PASS=${POSTGRES_PASSWORD}

Here, env_file brings everything from .env, and environment maps selected variables to names your container expects (optionally with defaults using the ${VAR:-default} form). This is the pattern used in this repo.

3) Only map what you need (no env_file)
services:
  your-service:
    image: your/image
    environment:
      - POSTGRES_USER=${POSTGRES_USER}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

This injects only the specified variables. Compose still reads .env to substitute ${...} in the compose file.

Healthcheck and shell expansion tip
- In YAML strings executed by the container shell, a single $ would be interpolated by Compose (variable substitution), and then the shell in the container may also expand it. If you want the variable to be expanded at runtime inside the container (not by Compose), escape the dollar sign with another dollar. Example already used here:

  healthcheck:
    test: ["CMD-SHELL", "pg_isready -U $${POSTGRES_USER}"]

The double $$ prevents Compose from substituting and leaves $POSTGRES_USER for the container to expand.

Where .env is read from
- Compose automatically loads .env from the same directory as docker-compose.yaml. Keep this file there.
- You can also export variables in your shell; shell environment has higher precedence than .env for substitution in the compose file.

Order of precedence (high → low)
1) Values set in the shell environment (e.g., export POSTGRES_USER=foo before docker compose up)
2) Values in the .env file
3) Defaults provided inline in docker-compose.yaml using ${VAR:-default}

Gotchas and tips
- Quotes: Values are taken as literal strings from .env (no YAML parsing). Avoid surrounding values with quotes in .env unless they’re part of the intended value.
- Booleans/numbers: They’re still strings inside the container unless your application parses them.
- Multi-line values: env_file does not support multi-line values. For long certs/keys, consider Docker secrets or mount files instead.
- Do not commit real secrets: Use scripts/generate-secrets.sh to generate random secrets locally and keep .env out of VCS.

Examples from this repo
- Postgres service reads its credentials from .env and uses them both for the container environment and in the init/healthcheck:

  services:
    postgres:
      env_file:
        - .env
      environment:
        - POSTGRES_DB=${POSTGRES_DB}
        - POSTGRES_HOST=${POSTGRES_HOST}
        - POSTGRES_PORT=${POSTGRES_PORT}
        - POSTGRES_USER=${POSTGRES_USER}
        - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
      healthcheck:
        test: ["CMD-SHELL", "pg_isready -U $${POSTGRES_USER}"]

- Authelia service injects its secrets from .env:

  services:
    authelia:
      env_file:
        - .env
      environment:
        - AUTHELIA_JWT_SECRET=${AUTHELIA_JWT_SECRET}
        - AUTHELIA_SESSION_SECRET=${AUTHELIA_SESSION_SECRET}
        - AUTHELIA_STORAGE_ENCRYPTION_KEY=${AUTHELIA_STORAGE_ENCRYPTION_KEY}

With this setup, just edit .env (or run scripts/generate-secrets.sh) and start:

- docker compose up -d
- To apply changes: docker compose up -d <service-name>
