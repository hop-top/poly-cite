---
doc_type: engineering
subtype: adr
status: accepted
title: ADR-0001 Polyglot repo structure
summary: Keep Go, TypeScript, Python, Rust, and PHP implementations in one repository with shared fixtures.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - tool_authors
  - oss_contributors
  - ai_assistant
confidentiality: public
tags:
  - cite
  - adr
  - polyglot
---

# ADR-0001: Polyglot Repo Structure

## Status

Accepted

## Context

The original URI parsing and handler registration code lived as separate
Go-centric packages. The new package needs to serve a polyglot framework and
multiple app runtimes without language drift.

## Decision

Keep all language ports in one repository:

- `go/`
- `ts/`
- `py/`
- `rs/`
- `php/`
- `spec/`
- `docs/`

Shared fixtures in `spec/` define cross-language behavior.

## Consequences

Positive:

- Cross-language changes are reviewed together.
- CI can detect parity drift before release.
- Documentation can describe one product instead of two packages.

Negative:

- Release and CI workflows must handle multiple ecosystems.
- Contributors need clearer ownership boundaries.
- The repository can accumulate generated artifacts unless ignore rules and
  packaging rules stay strict.

## Validation

- Every language test suite consumes the shared URI fixture.
- CI runs all language tests.
- Public docs describe `cite` as a unified package, not separate parser
  and handler packages.

## Related

- [Architecture](../architecture.md)
- [URI Contract](../specs/cite-contract.md)
