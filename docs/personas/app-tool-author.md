---
doc_type: product
subtype: persona
status: draft
title: App tool author persona
summary: Persona for developers exposing app objects through custom URI schemes.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - tool_authors
  - vibe_coders
  - ai_assistant
confidentiality: public
tags:
  - cite
  - persona
  - app-tool-author
---

# App Tool Author

## Responsibilities

- Define app-specific URI schemes and namespaces.
- Register parsers, completers, and handlers.
- Keep object links stable in CLIs, docs, and assistant conversations.

## Goals

- Users can click or paste a URI and land on the right object.
- Shell completions suggest valid objects.
- Links survive refactors and app packaging changes.

## Pain Points

- Deep links often start as ad-hoc strings.
- Custom scheme registration is OS-specific.
- Handler identity rules are easy to under-specify.

## Success Criteria

- A URI can be parsed and completed in every supported language.
- Handler setup is generated from one `HandlerSpec`.
- Version and environment metadata are encoded without breaking existing links.

