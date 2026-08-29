#!/usr/bin/env bash
# post-create.sh — runs after devcontainer create.
# Mirrors `make setup` plus the corepack step CI performs for pnpm.
set -euo pipefail

echo "==> Enabling corepack (pnpm)"
corepack enable

echo "==> Installing TypeScript dependencies"
# CI=true: pnpm 11 refuses to purge a pre-existing node_modules without a TTY,
# and postCreateCommand has none. Scoped to this call so interactive shells in
# the container are unaffected.
(cd ts && CI=true pnpm install --ignore-scripts)

echo "==> Installing Python dependencies"
(cd py && python3 -m pip install -U pip pytest build)

echo "==> Installing PHP dependencies"
(cd php && composer install --no-interaction --no-progress)

echo "==> Warming Rust toolchain"
(cd rs && cargo fetch)

echo "==> Warming Go module cache"
(cd go && go mod download)

echo "Dev environment ready. Run: make check"
