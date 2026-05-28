---
doc_type: project
subtype: story
status: draft
title: US-0008 Register a URL scheme on Windows
summary: User story for Windows registry-based custom URL scheme registration.
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
  - windows
---

# Story: Register a URL Scheme on Windows

**System:** handle
**Personas:** [Windows App Developers](../personas/handler-windows-app-developers.md)

---

## User Goal

As a Go developer targeting Windows, I want to call `handle.Register("ctxt", appPath)` and have
the handler written to `HKCU\Software\Classes\ctxt` without requiring UAC elevation, so that
clicking `ctxt://` links in a browser opens my app.

---

## Context

Windows URL scheme handlers are registered as registry keys under
`HKCU\Software\Classes\<scheme>` (per-user, no elevation) or `HKLM\Software\Classes\<scheme>`
(system-wide, requires admin). handle targets HKCU by default for CLI/developer installs.

The implementation uses `golang.org/x/sys/windows/registry` (pure Go, no CGO).

---

## Acceptance Criteria

- [ ] `handle.Register("ctxt", `C:\...\ctxt.exe`)` writes correct keys to HKCU on Windows
- [ ] After registration, clicking `ctxt://test` in Edge/Chrome opens ctxt.exe
- [ ] Registration is idempotent: second call overwrites without error
- [ ] Invalid appPath (empty string) returns descriptive error
- [ ] Build tag `windows` restricts the implementation to Windows
- [ ] CGO is NOT required (`golang.org/x/sys/windows/registry` is pure Go)
- [ ] generate.Snippet("windows", ...) works on Linux/macOS CI (cross-platform)

---

## Implementation Notes

### Registry Keys Written

```
HKCU\Software\Classes\ctxt
  (Default) = "URL:ctxt Protocol"
  "URL Protocol" = ""

HKCU\Software\Classes\ctxt\shell\open\command
  (Default) = "\"C:\\path\\to\\ctxt.exe\" \"%1\""
```

### Pseudocode

```
func Register(scheme, appPath string) error:
    if appPath == "":
        return errors.New("handle: appPath required on windows")
    root = registry.CURRENT_USER
    key  = `Software\Classes\` + scheme
    k, _, err = registry.CreateKey(root, key, registry.SET_VALUE)
    if err != nil: return wrap(err)
    defer k.Close()
    k.SetStringValue("", "URL:" + scheme + " Protocol")
    k.SetStringValue("URL Protocol", "")

    cmdKey, _, err = registry.CreateKey(root, key+`\shell\open\command`, registry.SET_VALUE)
    if err != nil: return wrap(err)
    defer cmdKey.Close()
    cmdKey.SetStringValue("", `"` + appPath + `" "%1"`)
    return nil
```

### File: register_windows.go

```go
//go:build windows
```

Uses `golang.org/x/sys/windows/registry`; no CGO.

---

## E2E Test Checklist

- [ ] After Register(), `reg query HKCU\Software\Classes\ctxt` shows expected keys
- [ ] `(Default)` value is `URL:ctxt Protocol`
- [ ] `shell\open\command (Default)` contains correct escaped path
- [ ] Clicking `ctxt://test` in Edge opens ctxt.exe (manual verification)
- [ ] Second call with same args returns nil (idempotent)
- [ ] `generate.Snippet("windows", "ctxt", `C:\...\ctxt.exe`)` compiles and runs on Linux
- [ ] Build with `GOOS=linux go build ./...` still succeeds (windows tag excluded)

---

## Related Stories

- [US-0009](./US-0009-unified-snippet-any-platform.md) — generate.Snippet() for .reg files
- [US-0010](./US-0010-handle-errunsupported.md) — ErrUnsupported on unsupported platforms
