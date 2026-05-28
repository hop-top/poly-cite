---
doc_type: research
subtype: stress_test
status: draft
title: URI contract stress test
summary: Stress-test notes for namespace, handler, version, and mode edge cases in the URI contract.
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
  - stress-test
  - research
---

# URI Contract Stress Test

## Objective

Test whether the proposed URI and handler identity model survives multi-app,
multi-language, multi-version, and assistant-generated usage.

## Scenarios

| Scenario | Expected behavior |
| --- | --- |
| Two apps both use `task` | Namespace separates object domains; OS handler ownership still needs policy |
| One app installed in Go and Python | Handler artifacts differ by language suffix |
| One app has dev and prod instances | Use `instance` for artifact identity; use query/channel metadata for URI interpretation |
| One object schema changes | Use `?schema=2`; keep scheme stable |
| Stress-test flow needs separate OS handler | Use `task-stress` only if routing must differ |
| Assistant invents `task://T-1` | Parser rejects missing namespace |
| Repo namespace is shorter than task namespace | Scheme policy sets `repo` namespace segments to `1` |

## Findings

- `scheme + value` is insufficient because it cannot distinguish namespace from
  object id.
- Version does not belong in the scheme for normal protocol evolution.
- Handler identity must be stricter than URI identity because artifacts live on
  local operating systems.
- Windows cannot fully solve multi-owner schemes without a dispatcher.
- Proto can help fixture consistency, but generated public APIs would make the
  package harder to use idiomatically.

## Recommendations

- Keep the URI model as `scheme + namespace + id + query + fragment`.
- Keep version and mode as metadata by default.
- Add language suffix to every generated handler artifact.
- Require shared fixture tests in every language.
- Document Windows scheme ownership limitations explicitly.

