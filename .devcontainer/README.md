# Development Environment Setup

This guide covers setting up the development environment for `hop.top/cite`.

`cite` spans five languages, so a local setup means five toolchains. The
devcontainer provides all of them at the versions CI uses.

## Recommended: VS Code Dev Containers

1. Install the **Dev Containers** extension in VS Code.
2. Open the project folder in VS Code.
3. Click "Reopen in Container" when prompted.

GitHub Codespaces works the same way with no local Docker install.

### What's included

| Tool | Version | Matches |
| --- | --- | --- |
| Go | 1.26.1 | `go/go.mod` |
| Node | 22 | CI matrix (also tested on 24) |
| pnpm | via corepack | `ts/package.json` |
| Python | 3.11 | CI |
| Rust | stable + `rustfmt`, `clippy` | CI |
| PHP | 8.2 + Composer | `php/composer.json` |
| `gh`, `make` | latest | — |

### Xdebug

The PHP feature ships Xdebug with `xdebug.mode=debug` and
`start_with_request=yes`, so every PHP call tries to reach a debug client on
port 9003 and prints a warning when none is listening. The container sets
`XDEBUG_MODE=off` to keep PHP output clean.

To debug PHP, override it for the run: `XDEBUG_MODE=debug php ...`.

### Post-creation automation

`post-create.sh` runs automatically and mirrors `make setup`:

- enables corepack (pnpm)
- installs TypeScript dependencies (`ts/`)
- installs Python build/test dependencies (`py/`)
- installs PHP dependencies (`php/`)
- warms the Rust and Go module caches

## Manual setup

Install Go 1.26.1, Node 22, Python 3.11, Rust stable, and PHP 8.2 with
Composer, then:

```sh
make setup
```

## Driving the container from the host

The Makefile wraps the `devcontainer` CLI (`npm i -g @devcontainers/cli`):

```sh
make devcontainer-build   # build image + run postCreate
make devcontainer-check   # run `make check` inside the container
make devcontainer-shell   # open a shell inside the container
make devcontainer-clean   # remove the container
```

These are for running the container from outside. Inside the container (VS
Code or Codespaces) use the plain targets below.

Note: `make devcontainer-check` fails at `lint-whitespace` when the workspace
is a `git hop` worktree — a worktree's `.git` is a pointer to a host path the
container cannot see, so `git diff --check` cannot resolve it. A normal clone
is unaffected.

## Verification

Run the docs lint, every language test suite, and every build:

```sh
make check
```

Language-specific targets: `make test-go`, `test-ts`, `test-py`, `test-rs`,
`test-php`, and `test-parity` for the cross-language fixture checks.
