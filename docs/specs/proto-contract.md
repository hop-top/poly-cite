---
doc_type: engineering
subtype: spec
status: draft
title: Proto contract boundary
summary: Contract for using protobuf as a shared fixture and wire schema without replacing handwritten language facades.
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
  - protobuf
  - facade
---

# Specification: Proto Contract Boundary

## Overview

`cite` may define a protobuf schema for fixtures, interop, and generated
test data. The proto is not the public API. Each language keeps a handwritten,
idiomatic facade.

## Requirements

1. Proto messages MAY define `URI`, `NamespacePolicy`, `HandlerSpec`, and test
   fixtures.
2. Generated code MUST stay behind package-internal adapters unless a language
   ecosystem requires otherwise.
3. Public parser and handler facades MUST remain handwritten.
4. Language implementations SHOULD use popular URL and OS registration
   dependencies behind replaceable interfaces.
5. A dependency replacement MUST NOT require a breaking public API change.

## Proposed Messages

```proto
message URI {
  string scheme = 1;
  string namespace = 2;
  string id = 3;
  string query = 4;
  string fragment = 5;
  string original = 6;
  string action = 7;
}

message NamespacePolicy {
  uint32 default_namespace_segments = 1;
  map<string, uint32> scheme_namespace_segments = 2;
}

message ActionRoute {
  string command = 1;
  repeated string args = 2;
}

message HandlerSpec {
  string vendor = 1;
  string app = 2;
  string instance = 3;
  string language = 4;
  string scheme = 5;
  string version = 6;
  string channel = 7;
  string app_path = 8;
  string display_name = 9;
}
```

## Facade Interfaces

```text
ParserBackend
  parse(input, policy, options) -> URI
  resolveAction(parsed, policy) -> ResolvedAction

HandlerBackend
  render(spec, target_os) -> HandlerArtifact
  register(spec, target_os) -> RegistrationResult
```

## Rationale

The proto gives all ports a common contract, but handwritten facades let each
language use its ecosystem's preferred URL parser, package conventions, and OS
integration libraries.
