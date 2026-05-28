---
doc_type: project
subtype: story
status: draft
title: US-0007 Generate a desktop file for Linux packaging
summary: User story for generating freedesktop desktop files for URL scheme handlers.
owner: Idea Crafters Labs
created: 2026-03-15
updated: 2026-05-13
audience:
  - developers
  - operations
  - tool_authors
  - ai_assistant
confidentiality: public
tags:
  - handle
  - story
  - linux
  - packaging
---

# Story: Generate a .desktop File for Linux Packaging

**System:** handle/generate
**Personas:** [Linux Package Maintainers](../personas/handler-linux-package-maintainers.md)

---

## User Goal

As a Linux package maintainer, I want to call `generate.Snippet("linux", "ctxt", "/usr/bin/ctxt")`
and receive a freedesktop.org-compliant .desktop file that registers `ctxt://` as a URL scheme
handler, so I can include it in deb/rpm/flatpak packages without writing the file by hand.

---

## Context

Linux URL scheme registration uses two components:
1. A `.desktop` file in `/usr/share/applications/` declaring `MimeType=x-scheme-handler/ctxt`
2. `xdg-mime default ctxt.desktop x-scheme-handler/ctxt` to activate it

The .desktop file must follow the freedesktop.org Desktop Entry Specification. Hand-crafted
files often omit `%u` in `Exec=` or set `NoDisplay=false`, causing broken or intrusive entries.

---

## Acceptance Criteria

- [ ] `generate.Snippet("linux", "ctxt", "/usr/bin/ctxt")` returns a valid .desktop string
- [ ] Output contains `MimeType=x-scheme-handler/ctxt;`
- [ ] Output contains `Exec=/usr/bin/ctxt %u`
- [ ] Output contains `NoDisplay=true`
- [ ] Output contains `Type=Application` and `Name=ctxt`
- [ ] Output passes `desktop-file-validate` with no errors
- [ ] Compiles on any GOOS (CGO-free)

---

## Implementation Notes

### Pseudocode

```
func desktopFileSnippet(scheme, appPath string) (string, error):
    if appPath == "":
        return "", fmt.Errorf("generate: appPath required for linux .desktop")
    tmpl = "[Desktop Entry]\nType=Application\nName={{scheme}}\n" +
           "Exec={{appPath}} %u\nMimeType=x-scheme-handler/{{scheme}};\nNoDisplay=true\n"
    return render(tmpl, scheme, appPath), nil
```

### File: generate/desktop.go

No build constraints; CGO-free; pure string/template rendering.

### Post-install Script Pattern

```bash
# /usr/share/applications/ctxt.desktop written by package
# postinst calls xdg-mime to activate:
xdg-mime default ctxt.desktop x-scheme-handler/ctxt
update-desktop-database /usr/share/applications
```

---

## E2E Test Checklist

- [ ] `generate.Snippet("linux", "ctxt", "/usr/bin/ctxt")` returns non-empty, nil error
- [ ] Output string contains `MimeType=x-scheme-handler/ctxt;`
- [ ] Output string contains `Exec=/usr/bin/ctxt %u`
- [ ] Output passes `desktop-file-validate --no-hints` on Ubuntu/Debian CI
- [ ] Writing output to `/usr/share/applications/ctxt.desktop` + running xdg-mime +
  update-desktop-database makes `xdg-open ctxt://test` launch the app
- [ ] `generate.Snippet("linux", "ctxt", "")` returns descriptive error (appPath required)
- [ ] Compiles on macOS/Windows CI (`GOOS=linux go build ./generate/...`)

---

## Related Stories

- [US-0009](./US-0009-unified-snippet-any-platform.md) — Unified Snippet() across platforms
- [US-0010](./US-0010-handle-errunsupported.md) — ErrUnsupported on unsupported platforms
