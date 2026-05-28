---
doc_type: project
subtype: story
status: draft
title: US-0006 Generate an Info.plist snippet for an iOS app
summary: User story for generating iOS static URL scheme plist snippets.
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
  - ios
  - plist
---

# Story: Generate an Info.plist Snippet for an iOS App

**System:** handle/generate
**Personas:** [App Bundle Developers](../personas/handler-bundle-developers.md)

---

## User Goal

As a Go developer building an iOS app, I want to call `generate.Snippet("ios", "ctxt", "")` and
receive a valid CFBundleURLTypes XML fragment I can embed in my Info.plist, because iOS has no
runtime URL scheme registration API.

---

## Context

iOS requires URL scheme handlers to be declared statically in Info.plist before App Store
submission. There is no runtime API equivalent to macOS's LSSetDefaultHandlerForURLScheme.
Developers using Go build tooling (e.g. gomobile, Fastlane) need a way to generate the
correct plist fragment without using Xcode directly.

---

## Acceptance Criteria

- [ ] `generate.Snippet("ios", "ctxt", "")` returns a non-empty string with no error
- [ ] Output contains `CFBundleURLTypes`, `CFBundleURLSchemes`, and the scheme value `ctxt`
- [ ] Output is valid XML (parseable by `encoding/xml` or `plutil -lint`)
- [ ] Output differs from macOS variant only in platform-appropriate keys (not structurally)
- [ ] appPath is ignored for iOS (schemes don't reference an executable path in plist)
- [ ] Multiple schemes: `generate.Snippet("ios", "ctxt myapp", "")` produces both entries
- [ ] generate subpackage compiles without CGO on any GOOS (Linux, macOS, Windows)

---

## Implementation Notes

### Pseudocode

```
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

### plistSnippet Output

```xml
<key>CFBundleURLTypes</key>
<array>
  <dict>
    <key>CFBundleURLSchemes</key>
    <array>
      <string>ctxt</string>
    </array>
    <key>CFBundleURLName</key>
    <string>top.hop.ctxt</string>
  </dict>
</array>
```

### File: generate/plist.go

No build constraints; CGO-free; pure string/template rendering.

---

## E2E Test Checklist

- [ ] `generate.Snippet("ios", "ctxt", "")` returns non-empty string, nil error
- [ ] Output string contains `<string>ctxt</string>`
- [ ] Output parses as valid XML without error
- [ ] `generate.Snippet("ios", "ctxt", "/usr/bin/ctxt")` ignores appPath; same output
- [ ] `generate.Snippet("macos", "ctxt", "")` returns structurally similar output
- [ ] Compiles on Linux CI (`GOOS=darwin GOARCH=arm64 go build ./generate/...`)

---

## Related Stories

- [US-0005](./US-0005-register-scheme-runtime-macos.md) — Runtime equivalent for macOS CLI
- [US-0009](./US-0009-unified-snippet-any-platform.md) — Unified Snippet() across platforms
