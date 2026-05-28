---
doc_type: project
subtype: story
status: draft
title: US-0010 Handle ErrUnsupported gracefully
summary: User story for detecting unsupported runtime URL scheme registration through a sentinel error.
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
  - story
  - errors
---

# Story: Handle ErrUnsupported Gracefully

**System:** handle
**Personas:** [CLI Tool Developers](../personas/handler-cli-tool-developers.md),
             [Platform Integrators](../personas/handler-platform-integrators.md)

---

## User Goal

As a Go developer calling `handle.Register()`, I want to detect when the current platform does not
support runtime URL scheme registration and degrade gracefully — without a panic, without a
generic error message, and without platform-specific branching in my own code.

---

## Context

Some platforms cannot support runtime URL scheme registration:
- iOS: scheme must be in Info.plist at build time; no runtime API
- Sandboxed macOS apps: LSSetDefaultHandlerForURLScheme may be denied by the sandbox
- Plan 9, WASM, and other niche GOOS values: no OS-level URL dispatch mechanism

`ErrUnsupported` is a sentinel error defined in `register_unsupported.go` and also returned
by platform implementations when they detect a runtime constraint (e.g. sandbox denial).

---

## Acceptance Criteria

- [ ] `handle.Register()` on an unsupported GOOS (e.g. plan9) returns `handle.ErrUnsupported`
- [ ] `errors.Is(err, handle.ErrUnsupported)` returns true for the returned error
- [ ] Caller can branch on ErrUnsupported without importing any platform package
- [ ] ErrUnsupported is exported from the `handle` package (not a subpackage)
- [ ] Platform implementations that detect runtime unsupported conditions wrap ErrUnsupported
  (so `errors.Is()` unwraps correctly)
- [ ] `register_unsupported.go` has build constraint `!darwin && !linux && !windows`

---

## Implementation Notes

### Sentinel Definition

```go
// register_unsupported.go
//go:build !darwin && !linux && !windows

var ErrUnsupported = errors.New(
    "handle: URL scheme registration not supported on this platform")

func Register(scheme, appPath string) error {
    return ErrUnsupported
}
```

### Caller Pattern

```go
err := handle.Register("ctxt", appPath)
switch {
case err == nil:
    log.Println("URL scheme registered.")
case errors.Is(err, handle.ErrUnsupported):
    snippet, _ := generate.Snippet(runtime.GOOS, "ctxt", appPath)
    fmt.Println("Manual step required:\n" + snippet)
default:
    return fmt.Errorf("register: %w", err)
}
```

### Platform Wrapping (for sandboxed macOS)

```go
// register_darwin.go — sandbox detection
if isSandboxed() {
    return fmt.Errorf("handle: sandboxed app: %w", ErrUnsupported)
}
```

---

## E2E Test Checklist

- [ ] `GOOS=plan9 go build ./...` compiles without errors
- [ ] Unit test: mock ErrUnsupported branch; `errors.Is(handle.ErrUnsupported, handle.ErrUnsupported)` true
- [ ] Unit test: wrapped error `fmt.Errorf("x: %w", handle.ErrUnsupported)` unwraps correctly
- [ ] Integration: on macOS, sandboxed-app simulation returns ErrUnsupported-wrapped error
- [ ] Caller test: branch on ErrUnsupported → generate.Snippet() called; no panic

---

## Related Stories

- [US-0005](./US-0005-register-scheme-runtime-macos.md) — macOS runtime registration
- [US-0008](./US-0008-register-scheme-windows.md) — Windows registration
- [US-0009](./US-0009-unified-snippet-any-platform.md) — Fallback to generate.Snippet()
