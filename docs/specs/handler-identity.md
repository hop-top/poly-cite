---
doc_type: engineering
subtype: spec
status: draft
title: Handler identity
summary: Handler identity and OS artifact naming contract for custom URI scheme registrations.
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
  - handlers
  - os-registration
---

# Specification: Handler Identity

## Metadata

- **Status**: draft
- **Version**: 1
- **Last Updated**: 2026-05-13
- **Created By**: Idea Crafters Labs

## Overview

The OS routes by scheme, but generated artifacts must be unique by vendor, app,
optional instance, language, and scheme. Otherwise the Go, Rust, Python,
TypeScript, and PHP packages can generate the same file or metadata name.

## HandlerSpec

```text
HandlerSpec
  vendor: string
  app: string
  instance: string optional
  language: enum(go, ts, py, rs, php)
  scheme: string
  version: string optional
  channel: string optional
  app_path: string
  display_name: string
```

## Handler ID

```text
<vendor>.<app>[.<instance>].<language>.<scheme>
```

Examples:

- `hop-top.scheme.go.task`
- `hop-top.scheme.py.task`
- `hop-top.scheme.local-dev.rs.task`

## OS Artifact Mapping

| Platform | Routing key | Unique artifact field |
| --- | --- | --- |
| macOS | URL scheme | `CFBundleURLName = HandlerID` |
| Linux | desktop handler | desktop filename and `Name` include HandlerID |
| Windows | `Software\\Classes\\<scheme>` | display metadata and command include HandlerID where possible |

Windows remains scheme-global at the registry key level. The package cannot
make two independent apps own the same scheme at the same time without an
external dispatcher.

## Version Policy

Version is metadata by default. It MUST NOT be part of the handler ID unless the
caller explicitly asks for side-by-side handler artifacts for multiple versions.

Default behavior:

- `version` appears in comments or metadata.
- `version` does not change OS routing.
- `version` does not change handler ID.

Side-by-side behavior:

- Put version in `instance`, for example `v0-2-0-alpha-0`.
- Keep `version` metadata populated with the human-readable version.

## Validation

Handler generators MUST reject:

- Empty `vendor`, `app`, `language`, `scheme`, or `app_path`.
- Unsupported language values.
- Handler IDs with path separators.
- Artifact names that cannot be represented safely on the target OS.

## Open Questions

1. Should TypeScript use `ts` or `js` when the runtime artifact is JavaScript?
2. Should Windows generation produce dispatcher guidance when multiple apps
   claim the same scheme?

