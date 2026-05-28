---
doc_type: product
subtype: prd
status: draft
title: cite PRD
summary: Product requirements for a polyglot custom URI scheme parsing and handler registration package.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - startup_founders
  - tool_authors
  - oss_contributors
  - ai_assistant
confidentiality: public
tags:
  - cite
  - prd
  - polyglot
---

# cite PRD

## 1. Executive Summary

We are building `cite` for framework authors, tool authors, and AI
assistant runtimes that need custom URI schemes to identify local or remote
objects and route them to the right app handler. The package will provide a
consistent URI contract and language-scoped handler registration surface across
Go, TypeScript, Python, Rust, and PHP.

## 2. Problem Statement

### Who has this problem?

- Framework authors building multiple apps on one runtime.
- App/tool authors exposing deep links into tasks, docs, repos, tickets, runs,
  agents, reports, and other domain objects.
- AI assistants that need stable links they can parse, complete, register, and
  hand back to humans or tools.
- OSS contributors maintaining SDK parity across language ecosystems.

### What is the problem?

Custom URI schemes are globally visible at the OS routing layer but are often
specified locally by individual tools. Without a shared contract, schemes clash,
handlers overwrite each other, URI parsers diverge by language, and generated
OS registration files use unstable filenames.

### Why is it painful?

- Two apps can claim the same scheme with incompatible semantics.
- Two language packages can generate the same handler artifact name.
- A framework can host multiple apps, but handler identity may only encode the
  scheme.
- Version-specific installs can create ambiguity if version is forced into the
  scheme instead of carried as metadata.
- AI assistants cannot reliably classify or complete links when URI structure is
  under-specified.

## 3. Target Users & Personas

Primary personas:

- [Framework author](../personas/framework-author.md)
- [App/tool author](../personas/app-tool-author.md)
- [AI assistant integrator](../personas/ai-assistant-integrator.md)

Secondary personas:

- OSS contributor maintaining one language port.
- Platform integrator packaging URI handlers for desktop or server runtimes.

Jobs to be done:

- Define a URI scheme without clashing with neighboring apps.
- Parse a URI into stable fields across languages.
- Generate handler artifacts that do not overwrite artifacts from another
  language, app, or instance.
- Register completion logic for known URI namespaces and IDs.
- Preserve protocol evolution metadata without changing OS-level routing.

## 4. Strategic Context

Business goal:

- Provide a small, reusable primitive for AI-agent-first products that need
  durable object references and app routing.

Why now:

- Custom URI scheme tooling has historically been Go-centric and
  fragmented across separate parser and handler packages.
- `cite` consolidates parsing + handler registration behind one
  contract, with kit-style parity across all 5 languages.
- Multi-app frameworks and AI assistants need a stricter routing
  contract before public adoption.

## 5. Solution Overview

`cite` provides:

- A canonical URI model: `scheme + namespace + id + query + fragment`.
- Scheme-specific namespace policy through `namespace_segments`.
- Cross-language parser and registry APIs.
- Completion facades so CLIs and assistant tools can suggest valid references.
- Handler identity based on vendor, app, optional instance, language, and
  scheme.
- OS-specific handler generation and registration surfaces.
- A fixture/proto contract to keep language implementations aligned.

## 6. Success Metrics

Primary metric:

- All supported languages pass the shared URI contract fixture suite.

Secondary metrics:

- Handler artifact names are unique across language, app, instance, and scheme.
- Release CI runs Go, TypeScript, Python, Rust, and PHP test suites.
- Public docs explain when to encode version, mode, or schema in query metadata
  instead of the scheme name.
- No initial release ships a stale local changelog.

## 7. User Stories & Requirements

Core stories:

- [US-0001 Parse namespaced URIs consistently](../stories/US-0001-parse-namespaced-cite.md)
- [US-0002 Generate language-scoped handler artifacts](../stories/US-0002-generate-language-scoped-handler-artifacts.md)
- [US-0003 Keep polyglot ports contract-compatible](../stories/US-0003-polyglot-contract-parity.md)
- [US-0004 Preserve version and mode metadata without scheme sprawl](../stories/US-0004-version-and-mode-metadata.md)

Functional requirements:

- Parse valid custom URIs into scheme, namespace, id, query, and fragment.
- Reject URIs that cannot satisfy the namespace policy for their scheme.
- Expose language-specific handler suffixes and stable handler IDs.
- Permit apps to declare one or more schemes and completion providers.
- Support generated contract fixtures without forcing generated public APIs.

Non-functional requirements:

- Keep public facades stable.
- Hide parser and handler dependencies behind replaceable interfaces.
- Avoid OS handler artifact collisions.
- Keep language packages small and idiomatic.

## 8. Out Of Scope

- Global scheme registry service.
- OS-specific installation wizards.
- Browser extension protocol handling.
- Protocol version negotiation beyond query metadata and fixture validation.

## 9. Dependencies & Risks

Technical dependencies:

- Language-standard URL parsers where sufficient.
- OS-specific registration conventions for macOS, Linux, and Windows.
- Release CI for all supported languages.

Risks:

- Scheme semantics drift if implementations interpret namespace policy
  differently.
- Windows registry remains scheme-global even if generated metadata is unique.
- Generated proto types can leak into public API unless the facade boundary is
  explicit.

Mitigations:

- Shared JSON fixtures and future proto contract.
- ADRs documenting facade and dependency boundaries.
- Handler identity spec that separates OS routing from artifact naming.

## 10. Open Questions

- Which URI schemes ship as examples in v0.1.0-alpha.0?
- Which handler registration operations should be dry-run only by default?
- Should invalid query metadata be rejected by core parsing or by app-level
  validation?

