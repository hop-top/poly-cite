---
doc_type: project
subtype: story
status: draft
title: US-0009 Unified snippet generation for any platform
summary: User story for generating platform-specific static URL scheme snippets through one API.
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
  - snippets
  - platform
---

# Story: Get a Unified Snippet for Any Platform via generate.Snippet()

**System:** handle/generate
**Personas:** [Platform Integrators](../personas/handler-platform-integrators.md),
             [App Bundle Developers](../personas/handler-bundle-developers.md)

---

## User Goal

As a platform integrator embedding handle in a setup wizard, I want to call
`generate.Snippet(platform, scheme, appPath)` and receive the correct static config snippet
for any target platform, so I can display it to the user or write it to disk without
platform-specific branching in my code.

---

## Context

Different platforms require different static config formats:

| Platform | Format | Key fields |
|----------|--------|-----------|
| `ios` | XML plist fragment | CFBundleURLTypes |
| `macos` | XML plist fragment | CFBundleURLTypes |
| `linux` | .desktop INI | MimeType=x-scheme-handler/ |
| `windows` | .reg registry export | HKCU\Software\Classes\<scheme> |

Platform integrators (ctxt, tlc) need a single call that returns the right format based on
a runtime or compile-time platform string, without the caller importing platform-specific code.

---

## Acceptance Criteria

- [ ] `generate.Snippet("ios", scheme, "")` returns plist fragment
- [ ] `generate.Snippet("macos", scheme, "")` returns plist fragment
- [ ] `generate.Snippet("linux", scheme, appPath)` returns .desktop file content
- [ ] `generate.Snippet("windows", scheme, appPath)` returns .reg file content
- [ ] Unknown platform returns `("", error)` with message naming the platform
- [ ] All variants compile and run on any GOOS (CGO-free)
- [ ] Function signature is stable: `func Snippet(platform, scheme, appPath string) (string, error)`

---

## Implementation Notes

### Pseudocode

```
// generate/snippet.go

func Snippet(platform, scheme, appPath string) (string, error):
    switch platform:
    case "ios", "macos":
        return plistSnippet(scheme)
    case "linux":
        return desktopFileSnippet(scheme, appPath)
    case "windows":
        return windowsRegSnippet(scheme, appPath)
    default:
        return "", fmt.Errorf("generate: unsupported platform %q", platform)
```

### Caller Pattern (Platform Integrator)

```go
target := runtime.GOOS  // or from config: "ios", "macos", etc.
snippet, err := generate.Snippet(target, "ctxt", appPath)
if err != nil {
    // unknown platform: log + skip; or present manual instructions
    log.Printf("generate snippet: %v", err)
    return
}
fmt.Printf("Add to your app config:\n\n%s\n", snippet)
```

### File Structure

```
generate/
  snippet.go     -- Snippet() dispatcher
  plist.go       -- plistSnippet()
  desktop.go     -- desktopFileSnippet()
  windows_reg.go -- windowsRegSnippet()
```

All files: no build tags, no CGO.

---

## E2E Test Checklist

- [ ] `generate.Snippet("ios", "ctxt", "")` → non-empty string, nil error
- [ ] `generate.Snippet("macos", "ctxt", "")` → non-empty string, nil error
- [ ] `generate.Snippet("linux", "ctxt", "/usr/bin/ctxt")` → non-empty string, nil error
- [ ] `generate.Snippet("windows", "ctxt", `C:\ctxt.exe`)` → non-empty string, nil error
- [ ] `generate.Snippet("plan9", "ctxt", "")` → empty string, non-nil error
- [ ] All cases compile under `GOOS=linux`, `GOOS=darwin`, `GOOS=windows`
- [ ] No CGO flags in any generate/ file

---

## Related Stories

- [US-0005](./US-0005-register-scheme-runtime-macos.md) — Runtime registration (macOS)
- [US-0006](./US-0006-generate-plist-snippet-ios.md) — plist snippet (iOS)
- [US-0007](./US-0007-generate-desktop-file-linux.md) — .desktop snippet (Linux)
- [US-0008](./US-0008-register-scheme-windows.md) — registry snippet (Windows)
- [US-0010](./US-0010-handle-errunsupported.md) — ErrUnsupported graceful degradation
