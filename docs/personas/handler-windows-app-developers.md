---
doc_type: product
subtype: persona
status: draft
title: handle windows app developers persona
summary: Persona for Windows developers registering custom URL schemes through the registry or installer snippets.
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
  - persona
  - windows
---

# Persona: Windows App Developers

**Primary Role:** Go developers targeting Windows who need to register a custom URL scheme in
the Windows registry (HKCU or HKLM) or generate a .reg snippet for inclusion in an installer

**Status: NEW — not covered by ctxt personas (see UNCOVERED.md)**

---

## Goals

- Register a URL scheme handler under HKCU\Software\Classes\<scheme> at runtime
- Generate a .reg snippet for inclusion in NSIS, WiX, or Inno Setup installers
- Handle UAC and permission scope correctly (HKCU vs HKLM without elevation)
- Test registry manipulation in CI without touching the real Windows registry
- Support both 32-bit and 64-bit app paths in the registry

---

## Interaction Pattern

### Runtime registration (HKCU, no elevation needed)

```go
import "hop.top/cite/handle"

// Registers under HKCU\Software\Classes\ctxt
err := handle.Register("ctxt", `C:\Program Files\ctxt\ctxt.exe`)
if err != nil {
    log.Printf("registry: %v", err)
}
```

### Generate .reg snippet for installer

```go
import "hop.top/cite/handle/generate"

reg, err := generate.Snippet("windows", "ctxt", `C:\Program Files\ctxt\ctxt.exe`)
if err != nil {
    return err
}
// Embed in NSIS .nsh file or write to ctxt-scheme.reg
os.WriteFile("ctxt-scheme.reg", []byte(reg), 0644)
```

### Expected .reg snippet

```reg
Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\Software\Classes\ctxt]
@="URL:ctxt Protocol"
"URL Protocol"=""

[HKEY_CURRENT_USER\Software\Classes\ctxt\shell\open\command]
@="\"C:\\Program Files\\ctxt\\ctxt.exe\" \"%1\""
```

### Installer-time (elevated, HKLM)

```bash
# NSIS post-install: run as admin, write to HKLM
WriteRegStr HKLM "Software\Classes\ctxt" "" "URL:ctxt Protocol"
WriteRegStr HKLM "Software\Classes\ctxt" "URL Protocol" ""
WriteRegStr HKLM "Software\Classes\ctxt\shell\open\command" "" \
  '"$INSTDIR\ctxt.exe" "%1"'
```

---

## Key Pain Points

- **HKCU vs HKLM:** HKCU works without elevation (preferred); HKLM requires admin rights
  but is system-wide — callers must choose consciously
- **Path escaping:** backslashes in registry values must be doubled; easy to get wrong in
  generated snippets
- **Stale entries:** uninstaller must clean up registry keys; handle.Register() doesn't
  auto-register an uninstaller hook
- **CGO-free on Windows:** `golang.org/x/sys/windows/registry` is pure Go; no CGO required
- **CI on Linux:** .reg snippet generation must work on Linux (cross-platform build agents);
  only runtime registration requires Windows

---

## System Leverage

### HKCU by Default

handle.Register() writes to HKCU without elevation; works for single-user installs and
developer machines without UAC prompt.

### generate.Snippet("windows", ...) — Cross-Platform

.reg generation is pure Go; runs on Linux build agents for cross-compiled Windows targets.
No wine, no Windows SDK required.

### Path Quoting Handled Internally

handle escapes backslashes and quotes in Exec paths automatically; caller passes raw Go path.

### golang.org/x/sys/windows/registry — No CGO

Registry writes use the pure-Go windows/registry package; CGO not required on Windows.

---

## User Stories

- [US-0008](../stories/US-0008-register-scheme-windows.md) — Register URL scheme on Windows
- [US-0009](../stories/US-0009-unified-snippet-any-platform.md) — Unified snippet via generate.Snippet()
- [US-0010](../stories/US-0010-handle-errunsupported.md) — Handle ErrUnsupported gracefully

---

## Success Metrics

- **No elevation required:** HKCU registration succeeds without UAC prompt
- **Valid .reg syntax:** generated .reg file imports cleanly via `reg import ctxt-scheme.reg`
- **Cross-platform generation:** snippet generation passes tests on Linux CI with no Windows
  dependency
- **Clean uninstall path:** documentation explicitly describes registry keys to remove

---

## Collaboration with Other Personas

- **CLI Tool Developers:** Windows devs use the same Register() API; platform differences
  are internal to handle
- **Platform Integrators:** integrators embed Windows registration in setup wizards
- **Linux Package Maintainers:** Linux and Windows paths are symmetric in the generate
  subpackage; same API, different output format
