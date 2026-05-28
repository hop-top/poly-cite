---
doc_type: product
subtype: persona
status: draft
title: handle app bundle developers persona
summary: Persona for macOS and iOS app bundle developers generating static URL scheme snippets.
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
  - app-bundle
---

# Persona: App Bundle Developers (Static Snippet Generation)

**Primary Role:** Go developers building macOS app bundles or iOS apps that require URL scheme
declaration in static config files (Info.plist), not runtime registration

---

## Goals

- Generate a correct Info.plist CFBundleURLTypes snippet for any scheme
- Embed the snippet in a Xcode project, build script, or Fastlane lane without manual editing
- Validate the snippet compiles/parses before shipping it
- Support both macOS (app bundle) and iOS (UIApplicationDelegate) variants
- Stay in Go tooling — no Xcode required at snippet-generation time

---

## Interaction Pattern

### Generate plist snippet in Go

```go
import "hop.top/cite/handle/generate"

snippet, err := generate.Snippet("macos", "ctxt", "/Applications/ctxt.app")
if err != nil {
    return err
}
// embed snippet in Info.plist via template or string injection
fmt.Println(snippet)
```

### CLI (via ctxt or handle tool)

```bash
# Emit snippet to stdout; pipe into a build script
handle generate plist --scheme ctxt --platform macos
handle generate plist --scheme ctxt --platform ios
```

### Fastlane / CI Integration

```ruby
# Fastlane lane reads snippet from handle, patches Info.plist
sh("handle generate plist --scheme ctxt --platform ios >> InfoPlist.strings")
```

### Expected Output (macOS plist fragment)

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

---

## Key Pain Points

- **iOS cannot use runtime registration:** scheme must be in Info.plist before app submission;
  no runtime API exists
- **plist hand-editing is error-prone:** mismatched XML tags break App Store submission
- **Multiple schemes:** apps may need 2–3 custom schemes; snippet must handle arrays
- **Variant confusion:** macOS bundle plist differs subtly from iOS Info.plist structure
- **Build pipeline coupling:** Xcode build phases are hard to script from Go tooling

---

## System Leverage

### generate.Snippet() — Platform-Targeted Output

Returns platform-specific static config (plist, .desktop, .reg) as a string.
Caller decides where to write it; handle never touches the filesystem directly.

### Validated Output

Snippet is well-formed XML/INI/reg before return; parse error surfaces before shipping.

### No Xcode Required

Pure Go; runs in headless CI, Docker, Linux build agents.
Generates iOS-compatible plist without an Apple SDK present.

---

## User Stories

- [US-0006](../stories/US-0006-generate-plist-snippet-ios.md) — Generate Info.plist snippet for iOS app
- [US-0009](../stories/US-0009-unified-snippet-any-platform.md) — Unified snippet via generate.Snippet()

---

## Success Metrics

- **Zero Xcode dependency:** snippet generation works in headless Linux CI
- **Parse-valid output:** generated plist passes `plutil -lint` with no errors
- **Round-trip parity:** snippet inserted into Info.plist produces identical URL routing as
  runtime registration on macOS
- **Multi-scheme support:** generate handles ≥2 schemes without truncation

---

## Collaboration with Other Personas

- **CLI Tool Developers:** bundle devs produce the static config; CLI devs use runtime path
- **Platform Integrators:** may call generate.Snippet() internally and expose higher-level API
- **Linux Package Maintainers:** .desktop output is the Linux equivalent of this workflow
