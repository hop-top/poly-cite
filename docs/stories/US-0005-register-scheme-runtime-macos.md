---
doc_type: project
subtype: story
status: draft
title: US-0005 Register a URL scheme at runtime on macOS
summary: User story for runtime macOS URL scheme registration through handle.Register.
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
  - macos
---

# Story: Register a URL Scheme at Runtime on macOS

**System:** handle
**Personas:** [CLI Tool Developers](../personas/handler-cli-tool-developers.md)

---

## User Goal

As a Go CLI tool developer, I want to call `handle.Register("ctxt", appPath)` on macOS and have
the OS route `ctxt://` links to my app, without writing any platform-specific code myself.

---

## Context

macOS exposes `LSSetDefaultHandlerForURLScheme` via the Launch Services framework. Calling it
requires CGO and the CoreServices umbrella framework. Developers building Go CLI tools should
not need to know this; handle should encapsulate it behind a single function call.

---

## Acceptance Criteria

- [ ] `handle.Register("ctxt", "/Applications/ctxt.app")` returns nil on success
- [ ] After registration, clicking a `ctxt://` link in Safari/Chrome opens the app
- [ ] Calling Register twice with the same args is idempotent (no error, no duplicate entry)
- [ ] Calling Register with an invalid/non-existent appPath returns a descriptive error
- [ ] Build tag `darwin` restricts the implementation to macOS
- [ ] CGO dependency is documented (godoc + README)

---

## Implementation Notes

### Pseudocode

```
func Register(scheme, appPath string) error:
    bundleID = resolveBundleID(appPath)   // e.g. "top.hop.ctxt"
    result   = LSSetDefaultHandlerForURLScheme(scheme, bundleID)
    if result != noErr:
        return fmt.Errorf("LSSetDefaultHandlerForURLScheme(%q): %d", scheme, result)
    return nil
```

### File: register_darwin.go

```go
//go:build darwin
```

Uses `#cgo LDFLAGS: -framework CoreServices` and calls `LSSetDefaultHandlerForURLScheme`.

### App Path Resolution

- If appPath ends in `.app`: read CFBundleIdentifier from
  `<appPath>/Contents/Info.plist`
- If appPath is a bare executable: derive bundle ID as `top.hop.<basename>`

---

## E2E Test Checklist

- [ ] `handle.Register("ctxt", "/Applications/ctxt.app")` returns nil
- [ ] `mdls -name kMDItemCFBundleIdentifier /Applications/ctxt.app` matches registered ID
- [ ] `lsregister -dump | grep ctxt` shows registered handler
- [ ] Clicking `ctxt://test` in default browser opens ctxt.app
- [ ] Second call with same args returns nil (idempotent)
- [ ] Call with non-existent path returns non-nil error containing path

---

## Related Stories

- [US-0009](./US-0009-unified-snippet-any-platform.md) — generate.Snippet() as fallback
- [US-0010](./US-0010-handle-errunsupported.md) — ErrUnsupported on unsupported platforms
