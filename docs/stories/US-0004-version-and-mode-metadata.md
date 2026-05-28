---
doc_type: project
subtype: story
status: draft
title: US-0004 Preserve version and mode metadata
summary: As an app author, I can encode schema, version, channel, or mode metadata without creating unnecessary schemes.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - tool_authors
  - ai_assistant
confidentiality: public
tags:
  - cite
  - story
  - metadata
---

# Story: Preserve Version And Mode Metadata

## Status

Draft

## Description

As an app author, I want schema version, app version, channel, and stress-test
mode to be metadata so that the URI scheme remains stable unless OS routing
must be separated.

## Acceptance Criteria

- `?schema=2` is preserved as query metadata.
- `?mode=stress` is preserved as query metadata.
- `task-dev` and `task-stress` are reserved for OS routing separation, not
  normal object versioning.
- Docs explain when to use query metadata versus a separate scheme.

## Technical Notes

- Version is not part of handler ID by default.
- Side-by-side installed versions use `instance` when artifact separation is
  required.

## Dependencies

- [URI Contract](../specs/cite-contract.md)
- [Handler Identity Spec](../specs/handler-identity.md)

## Related Features

- Stress testing
- Handler generation
- Protocol evolution

