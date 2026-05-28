---
doc_type: product
subtype: persona
status: draft
title: handle cli tool developers persona
summary: Persona for Go CLI developers registering custom URL schemes at runtime.
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
  - cli
---

# Persona: CLI Tool Developers (Runtime Registration)

**Primary Role:** Go developers building CLI tools that register a custom URL scheme at runtime

---

## Goals

- Register a URL scheme (e.g. `ctxt://`) at runtime so the OS routes clicks to their app
- Succeed or fail fast — no silent partial registration
- Handle errors gracefully; surface actionable messages
- Avoid platform-specific boilerplate; let handle handle OS differences
- Test registration logic without spawning a real OS handler

---

## Interaction Pattern

### Typical Workflow

```go
import "hop.top/cite/handle"

func main() {
    if err := handle.Register("ctxt", os.Executable()); err != nil {
        log.Fatalf("handle: register: %v", err)
    }
}
```

### Error Handling

```go
err := handle.Register("ctxt", appPath)
switch {
case errors.Is(err, handle.ErrUnsupported):
    fmt.Fprintln(os.Stderr, "URL scheme registration not supported here.")
    fmt.Fprintln(os.Stderr, "Use `ctxt generate snippet` for static config.")
case err != nil:
    return fmt.Errorf("register scheme: %w", err)
}
```

### Check Before Register (Idempotent Flow)

```go
// Safe to call on every startup; handle handles idempotency
if err := handle.Register("myapp", appPath); err != nil &&
    !errors.Is(err, handle.ErrUnsupported) {
    return err
}
```

---

## Key Pain Points

- **Platform divergence:** macOS uses LSSetDefaultHandlerForURLScheme; Linux needs xdg-mime +
  .desktop; Windows needs HKCU registry — different failure modes per OS
- **Silent failures:** OS may report success but handler never fires; hard to detect
- **Testing gap:** Can't test registration in CI (no display server, no LaunchServices)
- **Permission issues:** Linux: `xdg-mime` may not be in PATH; Windows: HKCU vs HKLM scope
- **Idempotency:** Calling Register twice shouldn't corrupt existing registration

---

## System Leverage

### Single Call Surface

`handle.Register(scheme, appPath)` — one function, all platforms.
No build tags required in caller code; handle owns them internally.

### Explicit Error Types

`ErrUnsupported` — caller can degrade gracefully (print snippet, skip registration).
Platform-specific errors wrapped with context for debuggability.

### Idempotent by Design

Re-registering the same scheme+path is a no-op or an update.
No accumulation of stale handlers.

### generate Subpackage Escape Hatch

When runtime registration is impossible (CI, sandboxed, iOS), caller falls through to
`generate.Snippet(...)` for static config output — same API shape.

---

## User Stories

- [US-0005](../stories/US-0005-register-scheme-runtime-macos.md) — Register URL scheme at runtime (macOS)
- [US-0008](../stories/US-0008-register-scheme-windows.md) — Register URL scheme on Windows
- [US-0010](../stories/US-0010-handle-errunsupported.md) — Handle ErrUnsupported gracefully

---

## Success Metrics

- **Zero boilerplate:** caller writes ≤5 lines to register a scheme
- **Error clarity:** all errors include platform context and a remediation hint
- **No silent failure:** if OS registration silently fails, handle surfaces a detectable error
- **CI-safe:** test suite passes without a running display server or LaunchServices daemon

---

## Collaboration with Other Personas

- **Platform Integrators:** embed handle; CLI devs are end-users of that embedding
- **Linux Package Maintainers:** CLI devs hand off .desktop generation; maintainers package it
- **Windows App Developers:** same Register() surface; Windows-specific caveats documented
