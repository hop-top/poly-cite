---
doc_type: engineering
subtype: adr
status: accepted
title: ADR-0005 Dependency facades
summary: Hide URL parser and OS handler dependencies behind replaceable backend interfaces.
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
  - dependencies
  - facade
---

# ADR-0005: Dependency Facades

## Status

Accepted

## Context

Each language has popular URL parsing and OS integration packages. Rewriting
all parsing and handler logic from scratch increases maintenance burden, but
exposing dependency types in the public API creates avoidable breakage.

## Decision

Use popular ecosystem dependencies behind stable facades:

- `ParserBackend`
- `HandlerBackend`

Dependency changes are implementation details unless behavior changes.

## Consequences

Positive:

- The package can adopt better dependencies without breaking users.
- Ports can stay idiomatic.
- Security or maintenance issues in dependencies can be addressed faster.

Negative:

- Facade tests must cover edge cases that dependencies handle differently.
- Adapters need clear error normalization.

## Validation

- Contract fixtures pass after dependency swaps.
- Public API signatures do not expose dependency-owned types.

## Related

- [Architecture](../architecture.md)
- [Proto Contract Boundary](../specs/proto-contract.md)
