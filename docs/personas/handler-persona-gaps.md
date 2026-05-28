---
doc_type: product
subtype: persona_gap_report
status: draft
title: handle uncovered personas
summary: Personas added for handle that were not covered by the source ctxt persona set.
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
  - gap-analysis
---

# Uncovered Personas (New in handle, Not Present in ctxt)

These personas were created for handle and have no direct analog in the ctxt persona set.

---

## windows-app-developers.md

**Why new:** ctxt has no Windows-specific persona. Windows URL scheme registration via the
Windows registry (HKCU\Software\Classes) is a distinct workflow with its own concerns:

- HKCU vs HKLM permission scope (no ctxt analog)
- Pure-Go `golang.org/x/sys/windows/registry` usage (no CGO, unlike macOS)
- .reg snippet generation for NSIS/WiX/Inno Setup installers
- Cross-platform .reg generation on Linux CI

ctxt's platform-integrators and operations personas were partially applicable but neither
addresses Windows-specific installer tooling or registry mechanics.

---

## Partially Adapted (Not New, But Significantly Diverged)

These personas were adapted from ctxt equivalents but diverged enough to note:

| handle persona | ctxt source | Key divergence |
|-------------|-------------|----------------|
| `cli-tool-developers.md` | `knowledge-workers.md` | handle users write Go; focus is registration API not search/capture |
| `bundle-developers.md` | `knowledge-workers.md` | Static config generation path; Xcode/iOS concerns absent from ctxt |
| `platform-integrators.md` | `platform-integrators.md` | Plugin interface replaced by minimal 2-function API; no plugin registry |
| `linux-package-maintainers.md` | `operations.md` | distro packaging (.desktop, xdg-mime) not present in ctxt ops persona |
