#!/usr/bin/env bash
set -euo pipefail
ENV_FILE="$(dirname "$0")/../.env"
mkdir -p "$(dirname "$ENV_FILE")"

rand() { head -c 64 /dev/urandom | base64 | tr -d '\n' | head -c "${1:-64}"; }

put() {
  local key="$1"; shift
  local value="$1"; shift || true
  if grep -q "^${key}=" "$ENV_FILE" 2>/dev/null; then
    # leave existing values intact
    echo "keep ${key}"
  else
    echo "${key}=${value}" >> "$ENV_FILE"
    echo "set  ${key}"
  fi
}

# Generate secrets if missing

put POSTGRES_USER postgres
put POSTGRES_PASSWORD "$(rand 48)"
put AUTHELIA_JWT_SECRET "$(rand 64)"
put AUTHELIA_SESSION_SECRET "$(rand 64)"
put AUTHELIA_STORAGE_ENCRYPTION_KEY "$(rand 64)"
put AUTHELIA_ADMIN_PASSWORD "$(rand 16)"
put BACKEND_AUTH_SECRET "$(rand 64)"

echo "Secrets written to $ENV_FILE"
