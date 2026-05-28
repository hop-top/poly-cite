---
doc_type: project
subtype: story
status: draft
title: US-0003 Keep polyglot ports contract-compatible
summary: As an OSS contributor, I can verify my language port against the same shared fixture suite as every other port.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - oss_contributors
  - ai_assistant
confidentiality: public
tags:
  - cite
  - story
  - parity
---

# Story: Keep Polyglot Ports Contract-Compatible

## Status

Draft

## Description

As an OSS contributor, I want one shared contract fixture so that I can update a
language port without guessing whether behavior matches the other ports.

## Acceptance Criteria

- Go, TypeScript, Python, Rust, and PHP tests load the same fixture file.
- CI fails if any language disagrees with expected fields.
- Public docs identify fixture changes as contract changes.
- Refactors that do not change fixture behavior do not bump release versions.

## Technical Notes

- Keep fixture data in `spec/fixtures/`.
- Add proto generation later without replacing handwritten facades.
- Run all language tests in CI before release.

## Dependencies

- [Proto Contract Boundary](../specs/proto-contract.md)
- [ADR-0001](../adrs/0001-polyglot-repo-structure.md)
- [ADR-0004](../adrs/0004-proto-contract-handwritten-facades.md)

## Related Features

- CI workflow
- Release-please setup
- Contract fixtures

