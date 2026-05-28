---
doc_type: engineering
subtype: spec
status: draft
title: URI contract
summary: Canonical URI parsing and normalization contract for all cite language implementations.
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
  - spec
  - parser
  - parity
---

# Specification: URI Contract

## Metadata

- **Status**: draft
- **Version**: 1
- **Last Updated**: 2026-05-13
- **Created By**: Idea Crafters Labs

## Overview

Every URI parsed by this package resolves to:

```text
scheme + namespace + id + query + fragment
```

The scheme selects the URI type and handler family. The namespace is the
collision domain. The id identifies an object inside the namespace. Query and
fragment are preserved metadata.

## Requirements

### Functional Requirements

1. A parser MUST reject empty input.
2. A parser MUST reject input without a scheme.
3. A parser MUST reject a URI whose namespace is empty.
4. A parser MUST reject a URI whose id is empty after applying namespace policy.
5. A parser MUST preserve query and fragment fields.
6. A parser MUST canonicalize without losing path segments.

### Namespace Policy

Each scheme declares how many leading path segments belong to the namespace.

| Scheme | Namespace segments | Example namespace | Example id |
| --- | ---: | --- | --- |
| `task` | 2 | `hop-top/cite` | `T-0001` |
| `doc` | 2 | `ideacrafters/kit` | `docs/adopters/quickstart` |
| `repo` | 1 | `hop-top` | `cite` |

Unknown schemes use `default_namespace_segments`, currently `1`.

### Canonical Form

```text
<scheme>://<namespace>/<id>[?<query>][#<fragment>]
```

Rules:

- The scheme is required.
- The namespace is slash-joined from the configured number of leading segments.
- The id is slash-joined from the remaining path segments.
- Query order is preserved by the parser unless a language dependency forces a
  normalized order; fixture tests must define accepted behavior before release.
- Fragment is preserved without the leading `#`.

### Version And Mode Metadata

Version, schema, channel, and stress mode are metadata, not scheme identity.

Use:

```text
task://hop-top/cite/T-0001?schema=2
task://hop-top/cite/T-0001?mode=stress&handler=go
```

Only use a separate scheme such as `task-dev` or `task-stress` when OS-level
routing must be isolated.

### Vanity Aliases

Vanity URIs are aliases to canonical URIs. They do not change identity.

Example:

```text
task://my-custom-slug/path-optional -> task://hop-top/cite/T-0001
```

Rules:

- A vanity alias MUST resolve to a canonical URI before namespace policy is
  applied.
- Exact vanity aliases MUST win over fuzzy matches.
- Prefix aliases MAY preserve the unmatched suffix when explicitly configured.
- Strict mode MUST disable fuzzy vanity fallback.
- Non-strict mode MAY fuzzy-match to the closest configured alias.
- If non-strict fuzzy matching produces multiple equally close aliases, the
  parser MUST return an ambiguity error.
- Implementations SHOULD support a machine-readable JSON ambiguity payload for
  callers that need to render choices.
- Completion integrations SHOULD show multiple fuzzy vanity candidates instead
  of silently selecting one when the fuzzy result is ambiguous.

### Action Routes

Apps MAY configure URI actions that resolve to command plans. Parsing MUST NOT
execute commands.

Canonical action query:

```text
tlc://org/repo/T-0001?action=task.claim
```

Supported aliases that normalize to the same action:

```text
tlc://org/repo/T-0001?cmd=task&verb=claim
tlc://org/repo/T-0001?name=task&action=claim
```

Example route:

```json
{
  "task.claim": {
    "command": "tlc",
    "args": ["-C", "{namespace}", "task", "claim", "{id}"]
  }
}
```

Rules:

- `action=<name.verb>` is the canonical query form.
- `cmd=<name>&verb=<verb>` and `name=<name>&action=<verb>` are accepted
  compatibility aliases.
- Alias query forms MUST normalize to the same action string, for example
  `task.claim`.
- Conflicting action query parameters MUST fail parsing.
- Resolved actions produce command plans only; callers decide whether to
  prompt, dry-run, authorize, or execute.
- Route templates MAY reference `{scheme}`, `{namespace}`, `{id}`, `{query}`,
  and `{fragment}`.

## Data Model

```text
URI
  scheme: string
  namespace: string
  id: string
  query: string
  fragment: string
  original: string
  action: string
```

## API Specifications

Every language SHOULD expose equivalent public operations:

- `parse(input, policy?, options?) -> URI`
- `URI.canonical() -> string`
- `URI.vanity() -> string`
- `Policy.resolveAction(parsed) -> ResolvedAction`
- `Registry.register(type, parser, completer?)`
- `Registry.parse(input) -> URI`
- `Registry.complete(prefix, context?) -> []Completion`
- `Registry.types() -> []string`

Language casing may be idiomatic, but semantics must match.

## Testing Strategy

Contract fixtures live in `spec/fixtures/cite-contract.json`.

Each language test suite MUST:

- Load the shared fixture.
- Assert all valid cases parse to expected fields.
- Assert all invalid cases fail.
- Assert canonical output for valid cases.

## Open Questions

1. Should query parameters be structured by the core package or left opaque?
2. Should parser policy be loaded from JSON in every language or compiled into
   tests only?
