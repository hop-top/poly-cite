# cite

Polyglot toolkit for custom URI schemes: parse, validate, register, and
complete `<scheme>://<namespace>/<id>` URIs from Go, TypeScript, Python,
Rust, and PHP — with a shared contract that keeps all 5 SDKs in lockstep.

## Install

```sh
go get hop.top/cite                # Go
pnpm add @hop-top/cite             # TypeScript
pip install hop-top-cite           # Python
cargo add hop-top-cite             # Rust
composer require hop-top/cite      # PHP
```

## Quick start

Parse a namespaced URI in Go (other SDKs in [Per-language usage](#per-language-usage)):

```go
package main

import (
    "fmt"
    "hop.top/cite/scheme"
)

func main() {
    parsed, _ := scheme.Parse("task://hop-top/cite/T-0001")
    fmt.Println(parsed.Namespace) // hop-top/cite
    fmt.Println(parsed.ID)        // T-0001
}
```

That's the minimum. From there: configure namespace policy per scheme,
register vanity aliases, resolve action routes, or generate OS handler
artifacts. See the per-SDK README for advanced examples in your language.

## What you get

- **`scheme`** — RFC-3986-adjacent parser. Configurable namespace
  segment count per scheme, vanity aliases (incl. fuzzy match), action
  routing, query + fragment surfacing.
- **`handle`** — OS handler artifact generation: `.desktop` (Linux),
  plist (macOS/iOS), registry snippets (Windows). Deterministic
  `handlerID()` so artifacts don't collide across languages.
- **`registry`** — register URI types with custom parsers and
  completers; query by scheme.
- **`completions`** — scheme-aware tab-completion candidate generation.
- **Cross-lang parity harness** — `make test-parity` runs all 5
  emitters against `spec/fixtures/*.json` and diffs outputs. CI gates
  on parity, so a fix in one SDK is enforceable across all 5.

## Per-language usage

Each SDK has its own README with install + idiomatic examples:

- [Go](go/README.md) — `hop.top/cite`
- [TypeScript](ts/README.md) — `@hop-top/cite`
- [Python](py/README.md) — `hop-top-cite`
- [Rust](rs/README.md) — `hop-top-cite`
- [PHP](php/README.md) — `hop-top/cite`

## How it's organized

| Path | Purpose |
| --- | --- |
| `go/` `ts/` `py/` `rs/` `php/` | Per-language SDK implementations |
| `spec/fixtures/` | Shared contract test data (loaded by all 5 SDKs) |
| `spec/proto/` | Protobuf schema (`hop.cite.v1`) |
| `docs/` | PRD, architecture, ADRs, specs, stories, glossary, personas |
| `tools/parity/` | Cross-lang parity test harness |

## Docs

- [PRD](docs/prd.md) — what cite is for, who it's for
- [Architecture](docs/architecture.md) — how the polyglot contract holds together
- [Make a custom URI scheme open your app](docs/guides/registering-a-scheme.md) — Linux, macOS, iOS, Windows install recipes
- [URI contract](docs/specs/cite-contract.md) — formal parser spec
- [Handler identity](docs/specs/handler-identity.md) — OS handler generation spec
- [Glossary](docs/glossary.md) — terminology
- [docs/](docs/README.md) — full index

## Status

Pre-1.0. API may shift across minor versions until 1.0. The
cross-language contract is enforced by `make test-parity` — any
breaking change to the contract is gated on all 5 SDKs.

## License

MIT. See [LICENSE](LICENSE).
