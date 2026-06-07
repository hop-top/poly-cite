---
doc_type: kb
subtype: index
status: draft
title: cite documentation index
summary: Audience-routed documentation map for the cite polyglot package.
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
  - documentation
  - index
---

# cite Documentation

`cite` is the unified polyglot package for custom URI scheme parsing,
completion, and OS handler registration.

## Read By Goal

| Goal | Start here |
| --- | --- |
| Understand the product | [PRD](prd.md) |
| Understand the architecture | [Architecture](architecture.md) |
| Make `<scheme>://` URLs open my app | [Registering a scheme](guides/registering-a-scheme.md) |
| Implement or review URI parsing | [URI Contract](specs/cite-contract.md) |
| Implement or review handler registration | [Handler Identity](specs/handler-identity.md) |
| Understand decisions | [ADRs](adrs/) |
| Implement a user-facing behavior | [Stories](stories/) |
| Understand users and jobs | [Personas](personas/) |
| Explain terms consistently | [Glossary](glossary.md) |

## Unified Structure

URI parsing, completions, and handler registration are documented together.
There is no separate `handle` documentation island; handler registration material
lives in the same `personas`, `stories`, `specs`, and `adrs` folders as
the rest of `cite`.

## Documentation Rules

All docs in this tree carry `.ops` frontmatter for indexing and policy checks:

- `doc_type`
- `subtype`
- `status`
- `title`
- `summary`
- `owner`
- `created`
- `updated`
- `audience`
- `confidentiality`
- `tags`
