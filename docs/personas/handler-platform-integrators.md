---
doc_type: product
subtype: persona
status: draft
title: handle platform integrators persona
summary: Persona for developers embedding URL scheme registration in larger setup or install flows.
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
  - integration
---

# Persona: Platform Integrators

**Primary Role:** Developers embedding handle in larger systems (e.g. ctxt, tlc, or third-party
Go apps) that need URL scheme registration as part of a broader install/setup flow

---

## Goals

- Embed handle as a library dependency with minimal coupling to OS-specific behaviour
- Expose a unified setup flow: "register scheme on this platform, generate snippet on that one"
- Handle platform-specific edge cases (unsupported, sandboxed, CI) without forking logic
- Surface registration status to end-users without leaking handle internals
- Keep handle version-pinned and updatable without breaking caller contracts

---

## Interaction Pattern

### Conditional Registration in an Install Command

```go
import (
    "errors"
    "hop.top/cite/handle"
    "hop.top/cite/handle/generate"
)

func installURLScheme(scheme, appPath, platform string) error {
    err := handle.Register(scheme, appPath)
    if errors.Is(err, handle.ErrUnsupported) {
        // Fall back to static snippet
        snippet, genErr := generate.Snippet(platform, scheme, appPath)
        if genErr != nil {
            return genErr
        }
        fmt.Printf("Add this to your app config:\n\n%s\n", snippet)
        return nil
    }
    return err
}
```

### Cross-Platform Setup Wizard

```go
// During first-run wizard, detect platform and take best path
switch runtime.GOOS {
case "darwin", "linux", "windows":
    if err := handle.Register(scheme, appPath); err != nil {
        return err
    }
    fmt.Println("URL scheme registered.")
default:
    snippet, _ := generate.Snippet(runtime.GOOS, scheme, appPath)
    fmt.Println("Manual step required:\n" + snippet)
}
```

### Integration Testing

```go
// In tests, verify snippet shape without touching real OS
snippet, err := generate.Snippet("linux", "ctxt", "/usr/bin/ctxt")
assert.NoError(t, err)
assert.Contains(t, snippet, "MimeType=x-scheme-handler/ctxt")
```

---

## Key Pain Points

- **API surface stability:** integrators need Register() and generate.Snippet() signatures to
  remain stable across handle minor versions
- **Error wrapping:** platform errors must be wrappable so integrators can add context
- **No global state:** handle must not use init() side-effects or package-level globals that
  interfere with multiple callers
- **CGO concerns:** macOS LSSetDefaultHandlerForURLScheme requires CGO; integrators must know
  upfront whether CGO is required for a given GOOS target
- **Testability:** must be mockable / skippable in unit tests (CI has no LaunchServices)

---

## System Leverage

### Minimal Public API

`handle.Register(scheme, appPath)` + `generate.Snippet(platform, scheme, appPath)` — two entry
points. Integrators depend on nothing else.

### ErrUnsupported as Sentinel

Predictable sentinel error; integrators can branch on it without string matching.

### No Init Side-Effects

handle performs no OS calls at package init time; safe to import without consequences.

### generate Is CGO-Free

Static snippet generation never calls OS APIs; always compiles without CGO.
Integrators can build generate-only paths for cross-compiled targets.

---

## User Stories

- [US-0009](../stories/US-0009-unified-snippet-any-platform.md) — Unified snippet via generate.Snippet()
- [US-0010](../stories/US-0010-handle-errunsupported.md) — Handle ErrUnsupported gracefully

---

## Success Metrics

- **Stable API:** no breaking changes to Register() or generate.Snippet() signatures in
  minor releases
- **Zero init() calls:** `import "hop.top/cite/handle"` causes zero OS interactions
- **CGO documented:** CGO requirement per platform is explicit in README and godoc
- **Mock-friendly:** test suite can stub OS registration without build constraints

---

## Collaboration with Other Personas

- **CLI Tool Developers:** integrators provide the setup flow; CLI devs call Register() directly
- **Bundle Developers:** integrators may expose plist generation in higher-level tooling
- **Linux Package Maintainers:** integrators hand off .desktop output to packaging scripts
- **Windows App Developers:** integrators surface registry snippets in Windows install flows
