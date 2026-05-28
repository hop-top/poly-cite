---
doc_type: product
subtype: persona
status: draft
title: handle linux package maintainers persona
summary: Persona for Linux maintainers packaging custom URL scheme handlers with desktop files and xdg-mime.
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
  - persona
  - linux
  - packaging
---

# Persona: Linux Package Maintainers

**Primary Role:** Distro maintainers and packagers who need .desktop files and xdg-mime
integration to register URL schemes as part of an OS package (deb, rpm, flatpak, snap)

---

## Goals

- Generate a standards-compliant .desktop file that registers a URL scheme handler
- Integrate xdg-mime registration into post-install/post-remove scripts
- Verify .desktop file is valid before packaging (avoid broken installs)
- Support both runtime registration (for CLI post-install hooks) and static file generation
  (for package build pipelines)
- Stay compatible with XDG Base Directory and freedesktop.org spec

---

## Interaction Pattern

### Generate .desktop file in build pipeline

```go
import "hop.top/cite/handle/generate"

desktop, err := generate.Snippet("linux", "ctxt", "/usr/bin/ctxt")
if err != nil {
    return err
}
// Write to /usr/share/applications/ctxt.desktop
os.WriteFile("/usr/share/applications/ctxt.desktop", []byte(desktop), 0644)
```

### Post-install hook (runtime registration)

```bash
# debian/postinst
handle register ctxt /usr/bin/ctxt
xdg-mime default ctxt.desktop x-scheme-handler/ctxt
update-desktop-database /usr/share/applications
```

### Via handle.Register() in Go installer

```go
// Called from post-install step of go-based installer
if err := handle.Register("ctxt", "/usr/bin/ctxt"); err != nil {
    // Log but don't fail package install; .desktop already written by package
    log.Printf("warn: xdg-mime registration failed: %v", err)
}
```

### Expected .desktop output

```ini
[Desktop Entry]
Type=Application
Name=ctxt
Exec=/usr/bin/ctxt %u
MimeType=x-scheme-handler/ctxt;
NoDisplay=true
```

---

## Key Pain Points

- **xdg-mime not always in PATH:** headless servers, minimal containers, CI; Register() must
  fail with clear error rather than silently succeed
- **update-desktop-database required:** without it, .desktop changes don't take effect; often
  forgotten in post-install scripts
- **Flatpak/snap sandboxing:** xdg-mime calls may be intercepted or denied; generate-only
  path must always work
- **Distro naming conventions:** .desktop filename must match app ID; variations cause
  duplicate entries in app menus
- **Exec= format:** `%u` flag for URL handling must be present; many hand-crafted .desktops
  omit it

---

## System Leverage

### generate.Snippet("linux", ...) — Spec-Compliant Output

Generates freedesktop.org-compliant .desktop file with correct MimeType= and Exec=%u fields.
Caller writes to the right path; handle doesn't assume install prefix.

### handle.Register() — xdg-mime Wrapper

Calls `xdg-mime default <desktop-file> x-scheme-handler/<scheme>` under the hood.
Returns descriptive error if xdg-mime is missing or returns non-zero.

### NoDisplay=true by Default

Registered handler doesn't pollute the application menu; only fires for scheme links.

---

## User Stories

- [US-0007](../stories/US-0007-generate-desktop-file-linux.md) — Generate .desktop file for Linux packaging

---

## Success Metrics

- **Spec compliance:** generated .desktop passes `desktop-file-validate` with no errors
- **xdg-mime success:** `xdg-mime query default x-scheme-handler/ctxt` returns correct app
  after handle.Register()
- **Clear failure on missing xdg-mime:** error message names the missing binary and
  suggests the manual command
- **Idempotent post-install:** running the hook twice doesn't duplicate MimeType entries

---

## Collaboration with Other Personas

- **CLI Tool Developers:** package maintainers ship the .desktop; CLI devs call Register()
  at runtime for dev installs
- **Platform Integrators:** integrators may generate .desktop files during app setup and
  hand them to package build pipelines
- **Bundle Developers:** .desktop is the Linux analog of Info.plist; same generate path,
  different output format
