---
doc_type: kb
subtype: index
status: draft
title: handle user personas index
summary: Index of handle personas and their primary URL scheme registration workflows.
owner: Idea Crafters Labs
created: 2026-03-15
updated: 2026-05-13
audience:
  - developers
  - tool_authors
  - ai_assistant
confidentiality: public
tags:
  - handle
  - personas
  - index
---

# handle — User Personas

Primary personas for `hop.top/cite/handle`, the cross-platform URL scheme registration package.

## Quick Overview

| Persona | File | Role | Primary Interaction |
|---------|------|------|-------------------|
| **CLI Tool Developers** | [`cli-tool-developers.md`](./handler-cli-tool-developers.md) | Go devs building CLI tools | `handle.Register()` at runtime |
| **App Bundle Developers** | [`bundle-developers.md`](./handler-bundle-developers.md) | macOS/iOS app bundle devs | `generate.Snippet()` for static plist |
| **Platform Integrators** | [`platform-integrators.md`](./handler-platform-integrators.md) | Devs embedding handle in larger systems | Conditional Register + Snippet |
| **Linux Package Maintainers** | [`linux-package-maintainers.md`](./handler-linux-package-maintainers.md) | Distro/package maintainers | .desktop generation + xdg-mime |
| **Windows App Developers** | [`windows-app-developers.md`](./handler-windows-app-developers.md) | Windows Go devs | HKCU registration + .reg snippets |

See [UNCOVERED.md](./UNCOVERED.md) for personas that are new (not adapted from ctxt).

## API Split by Persona

| Path | Package | Use |
|------|---------|-----|
| Runtime registration | `hop.top/cite/handle` | CLI devs, platform integrators, Windows devs |
| Static snippet generation | `hop.top/cite/handle/generate` | Bundle devs, Linux maintainers, integrators |
| Graceful degradation | `handle.ErrUnsupported` | All personas |
