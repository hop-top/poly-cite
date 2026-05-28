---
doc_type: engineering
subtype: adr
status: accepted
title: ADR-0004 Proto contract with handwritten facades
summary: Use proto for shared contracts and fixtures while keeping public APIs handwritten and idiomatic.
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
  - protobuf
  - facade
---

# ADR-0004: Proto Contract With Handwritten Facades

## Status

Accepted

## Context

A shared schema can prevent language drift, but generated public APIs tend to
feel non-idiomatic and can leak implementation details.

## Decision

Use proto as an internal contract and fixture generation source. Keep public
language APIs handwritten.

## Consequences

Positive:

- Shared model and fixtures remain explicit.
- Public APIs can follow language conventions.
- Generated-code churn does not become a user-facing breaking change.

Negative:

- Implementations must map between generated and handwritten types.
- Tests must enforce that mappings stay lossless.

## Validation

- Generated fixture data round-trips through every public facade.
- Public docs reference the facade API, not generated types.

## Related

- [Proto Contract Boundary](../specs/proto-contract.md)

