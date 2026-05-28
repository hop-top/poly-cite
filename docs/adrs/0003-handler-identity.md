---
doc_type: engineering
subtype: adr
status: accepted
title: ADR-0003 Handler identity
summary: Handler artifacts are unique by vendor, app, optional instance, language, and scheme.
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
  - handlers
---

# ADR-0003: Handler Identity

## Status

Accepted

## Context

Each language implementation can generate OS handler artifacts. If all
languages emit the same filename or bundle metadata, packages overwrite each
other.

## Decision

Use this handler ID:

```text
<vendor>.<app>[.<instance>].<language>.<scheme>
```

Version is metadata by default. If a caller needs side-by-side versions, it
puts the version-like value in `instance`.

## Consequences

Positive:

- Go, TypeScript, Python, Rust, and PHP artifacts do not collide.
- Multi-app frameworks can run separate app instances.
- Version routing remains opt-in rather than default scheme sprawl.

Negative:

- Windows remains scheme-global at the registry key.
- Callers must provide enough app identity metadata.

## Validation

- Handler generators reject incomplete specs.
- Linux desktop files and macOS URL metadata include the handler ID.
- Windows generator documents the scheme-global limitation.

## Related

- [Handler Identity Spec](../specs/handler-identity.md)

