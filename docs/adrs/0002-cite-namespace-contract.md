---
doc_type: engineering
subtype: adr
status: accepted
title: ADR-0002 URI namespace contract
summary: Use scheme-specific namespace segment policy to avoid object and app collisions.
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
  - namespace
---

# ADR-0002: URI Namespace Contract

## Status

Accepted

## Context

Custom URI schemes identify both a route family and an object reference. If the
package only models `scheme + value`, apps cannot reliably separate collision
domain from object ID.

## Decision

Model every URI as:

```text
scheme + namespace + id + query + fragment
```

Each scheme declares `namespace_segments`. The namespace is the leading segment
set, and the id is the remaining path.

## Consequences

Positive:

- App and organization boundaries are explicit.
- A framework can host multiple apps with predictable object references.
- AI assistants can classify, complete, and explain URIs consistently.

Negative:

- Parsers must understand scheme policy.
- Existing shorthand values may need migration.
- Unknown schemes need a default policy and clear error behavior.

## Validation

- Fixture cases cover `task`, `doc`, `repo`, `task-dev`, and `task-stress`.
- Invalid cases reject missing namespace or missing id.

## Related

- [URI Contract](../specs/cite-contract.md)
- [Stress Test](../stress-test-cite-contract.md)

