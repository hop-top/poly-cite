---
doc_type: project
subtype: story
status: draft
title: US-0006 Generate language-scoped handler artifacts
summary: As a framework author, I can generate OS handler artifacts that do not collide across language ports or app instances.
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
  - handlers
---

# Story: Generate Language-Scoped Handler Artifacts

## Status

Draft

## Description

As a framework author, I want handler artifacts to include vendor, app,
instance, language, and scheme so that multiple apps and language ports do not
overwrite each other on the same machine.

## Acceptance Criteria

- A Go handler for `task` emits an artifact identity ending in `.go.task`.
- A Python handler for the same app and scheme emits an identity ending in
  `.py.task`.
- A local development instance can add `.local-dev` without changing scheme
  routing.
- Generators reject missing vendor, app, language, scheme, or app path.

## Technical Notes

- macOS uses the handler ID in `CFBundleURLName`.
- Linux uses the handler ID in the desktop filename.
- Windows remains scheme-global and must document conflicts.

## Dependencies

- [Handler Identity Spec](../specs/handler-identity.md)
- [ADR-0003](../adrs/0003-handler-identity.md)

## Related Features

- OS registration
- Release packaging
- Multi-app framework integration

