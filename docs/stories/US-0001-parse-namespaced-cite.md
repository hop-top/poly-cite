---
doc_type: project
subtype: story
status: draft
title: US-0001 Parse namespaced URIs consistently
summary: As a tool author, I can parse a custom URI into scheme, namespace, id, query, and fragment consistently across languages.
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
  - story
  - parser
---

# Story: Parse Namespaced URIs Consistently

## Status

Draft

## Description

As a tool author, I want every language port to parse a custom URI into the
same fields so that my app can use stable links across CLIs, servers, agents,
and generated docs.

## Acceptance Criteria

- Given `task://hop-top/cite/T-0001`, every implementation returns
  `scheme=task`, `namespace=hop-top/cite`, and `id=T-0001`.
- Given `doc://ideacrafters/kit/docs/spec#section-2`, every implementation
  preserves `fragment=section-2`.
- Given `task://hop-top/cite/T-0001?schema=2`, every implementation
  preserves `query=schema=2`.
- Invalid fixture cases fail in every implementation.

## Technical Notes

- Use `namespace_segments` to split namespace from id.
- Keep query and fragment separate from id.
- Avoid public dependency-owned URL types.

## Dependencies

- [URI Contract](../specs/cite-contract.md)
- `spec/fixtures/cite-contract.json`
- [ADR-0002](../adrs/0002-cite-namespace-contract.md)

## Related Features

- Polyglot parity CI
- URI completions
- Handler routing

