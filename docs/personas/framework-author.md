---
doc_type: product
subtype: persona
status: draft
title: Framework author persona
summary: Persona for maintainers building a framework that hosts multiple URI-aware apps.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - startup_founders
  - tool_authors
  - ai_assistant
confidentiality: public
tags:
  - cite
  - persona
  - framework-author
---

# Framework Author

## Responsibilities

- Provide shared primitives for multiple apps.
- Keep URI routing stable across app versions and environments.
- Prevent handler collisions across languages and app instances.
- Document safe conventions for adopters.

## Goals

- One URI contract across all framework apps.
- App-specific namespaces without global scheme sprawl.
- Safe handler generation for developer machines and production packaging.

## Pain Points

- OS handlers are global enough to conflict.
- App versions and environments can be confused with schemes.
- Language ports drift unless fixtures are shared.

## Success Criteria

- New apps can define schemes and namespace policy without changing core code.
- AI assistants can parse and explain framework URIs.
- Handler artifacts remain unique by app and language.

