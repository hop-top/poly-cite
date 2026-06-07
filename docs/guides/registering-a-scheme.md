---
doc_type: kb
subtype: guide
status: draft
title: Registering a custom URI scheme on Linux, macOS, iOS, and Windows
summary: End-to-end recipe for taking a cite `Handle` snippet and getting the OS to route `<scheme>://...` URLs to your app.
owner: Idea Crafters Labs
created: 2026-06-06
updated: 2026-06-06
audience:
  - developers
  - tool_authors
  - operations
  - ai_assistant
confidentiality: public
tags:
  - cite
  - handle
  - guide
  - os-registration
  - linux
  - macos
  - ios
  - windows
---

# Registering a custom URI scheme

`cite` ships a `Handle` API in every SDK that produces the **artifact** an
operating system needs to route `<scheme>://...` URLs to your application:

| Platform | Artifact `Handle` produces | What you do with it |
| --- | --- | --- |
| Linux | `.desktop` file (freedesktop INI) | Write under `~/.local/share/applications/`, then `xdg-mime default` |
| macOS | `Info.plist` `CFBundleURLTypes` fragment | Merge into your `.app` bundle's `Info.plist` |
| iOS | Same `CFBundleURLTypes` fragment | Merge into your app's `Info.plist` before App Store submission |
| Windows | `.reg` registry export | `reg import`, or (Go only) call `handle.Register` to write directly |

The artifact format is the same across all five SDKs — they share a single
contract (see [`handler-identity.md`](../specs/handler-identity.md)) — so a
Python script and a Go binary that both want to own `task://` produce
identical files modulo the `language` field in the handler ID.

This guide assumes a running scheme called `task` routed to an executable.
Substitute your own scheme, vendor, app name, and path.

## Step 1 — Build a `HandlerSpec`

Every `Handle` call takes the same shape:

```text
HandlerSpec {
  vendor       // "hop-top"            — your org/namespace
  app          // "myapp"              — your application name
  language     // "go" | "ts" | "py" | "rs" | "php"
  scheme       // "task"               — the URI scheme you want
  appPath      // /usr/bin/myapp       — Linux/Windows: executable path
                                       — macOS/iOS:    bundle identifier
  displayName  // optional, shown in OS UIs
  instance     // optional, for side-by-side installs (e.g. "dev")
  version      // optional metadata
  channel      // optional metadata
}
```

`vendor`, `app`, `language`, `scheme`, and `appPath` are required. The
`language` field is what keeps the artifact filenames distinct when the
same scheme is registered by two SDKs at once — see
[`handler-identity.md`](../specs/handler-identity.md) for the rationale.

## Step 2 — Generate the snippet in your language

All five SDKs expose `snippet(platform, spec)` (or `Snippet(platform, spec)`
in Go) that returns the platform-appropriate artifact as a string. Valid
`platform` values are `"linux"`, `"macos"`, `"ios"`, `"windows"`.

### Go

```go
import "hop.top/cite/handle/generate"

spec := generate.HandlerSpec{
    Vendor:   "hop-top",
    App:      "myapp",
    Language: generate.LanguageGo,
    Scheme:   "task",
    AppPath:  "/usr/bin/myapp",
}
out, err := generate.Snippet("linux", spec)
```

### TypeScript

```ts
import { snippet } from "@hop-top/cite";

const out = snippet("linux", {
  vendor: "hop-top",
  app: "myapp",
  language: "ts",
  scheme: "task",
  appPath: "/usr/bin/myapp",
});
```

### Python

```python
from cite.handle import HandlerSpec, snippet

out = snippet("linux", HandlerSpec(
    vendor="hop-top",
    app="myapp",
    language="py",
    scheme="task",
    app_path="/usr/bin/myapp",
))
```

### Rust

```rust
use hop_top_cite::handle::{snippet, HandlerSpec, Language};

let spec = HandlerSpec {
    vendor: "hop-top".into(),
    app: "myapp".into(),
    language: Language::Rs,
    scheme: "task".into(),
    app_path: "/usr/bin/myapp".into(),
    ..Default::default()
};
let out = snippet("linux", &spec)?;
```

### PHP

```php
use HopTop\Cite\HandlerSpec;
use HopTop\Cite\Handle;

$spec = new HandlerSpec(
    vendor:   "hop-top",
    app:      "myapp",
    language: "php",
    scheme:   "task",
    appPath:  "/usr/bin/myapp",
);
$out = Handle::snippet("linux", $spec);
```

The remaining sections show what `out` looks like and how to install it
per platform.

## Linux

The `linux` snippet is a freedesktop.org Desktop Entry file:

```ini
[Desktop Entry]
Type=Application
Name=hop-top.myapp.go.task
Exec=/usr/bin/myapp %u
MimeType=x-scheme-handler/task;
NoDisplay=true
X-Hop-Handler-ID=hop-top.myapp.go.task
```

### Install (per-user, no root)

```sh
mkdir -p ~/.local/share/applications
printf '%s' "$out" > ~/.local/share/applications/hop-top.myapp.go.task.desktop
update-desktop-database ~/.local/share/applications              # refresh MIME cache
xdg-mime default hop-top.myapp.go.task.desktop x-scheme-handler/task
```

The filename must match what `desktop_filename(spec)` / `Handle::desktopFilename()`
returns — that's the same string as the handler ID with a `.desktop` suffix.

### Install (system-wide)

```sh
sudo cp myapp.desktop /usr/share/applications/
sudo update-desktop-database /usr/share/applications
```

### Verify

```sh
xdg-mime query default x-scheme-handler/task
# expected: hop-top.myapp.go.task.desktop

xdg-open task://hop-top/cite/T-0001
# expected: /usr/bin/myapp task://hop-top/cite/T-0001 launches
```

### Go shortcut

Go users on Linux can skip steps 2 + install entirely with the runtime
registrar — it writes the `.desktop` file and runs `xdg-mime` in one call:

```go
import "hop.top/cite/handle"

err := handle.Register("task", "/usr/bin/myapp")
```

This is Go-only. Other SDKs use the `snippet(...)` + manual install flow
described above.

## macOS

The `macos` snippet is a `CFBundleURLTypes` XML fragment:

```xml
<key>CFBundleURLTypes</key>
<array>
    <dict>
        <key>CFBundleURLName</key>
        <string>hop-top.myapp.go.task</string>
        <key>CFBundleURLSchemes</key>
        <array>
            <string>task</string>
        </array>
    </dict>
</array>
```

### Install

Merge the fragment into your `.app` bundle's `Contents/Info.plist`, inside
the top-level `<dict>`. The Go/TypeScript/Python/Rust/PHP SDKs all expose a
`patch_plist` / `patchPlist` / `PatchPlist` helper that does this for you:

```go
out, err := generate.PatchPlist(srcInfoPlistReader, spec)
```

Then ship the updated bundle. `LSSetDefaultHandlerForURLScheme` (called by
the OS when the bundle is registered with Launch Services — happens
automatically on first run, or manually via `lsregister`) makes your
bundle the default handler:

```sh
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister \
    -f /Applications/MyApp.app
```

### Verify

```sh
plutil -lint /Applications/MyApp.app/Contents/Info.plist
open task://hop-top/cite/T-0001
```

### Go shortcut

Go on macOS can register a running, already-installed bundle at runtime
via `LSSetDefaultHandlerForURLScheme` (uses CGO):

```go
err := handle.Register("task", "com.hop-top.myapp")  // bundle identifier
```

This sets the *user's* default, but the bundle's `Info.plist` still must
declare the scheme — runtime registration just marks which bundle wins
when multiple declare the same scheme.

## iOS

The `ios` snippet is identical in shape to macOS — same
`CFBundleURLTypes` fragment.

### Install

iOS has no runtime registration API. The plist fragment must be embedded
in your app's `Info.plist` at build time:

- **Xcode**: paste into `Info` tab → URL Types, or edit `Info.plist` source.
- **gomobile / Fastlane**: use `PatchPlist` to inject before code-signing.

After App Store submission and install, iOS routes `task://...` URLs to
your app. There is no command-line verification step on a real device;
test in the iOS Simulator with:

```sh
xcrun simctl openurl booted task://hop-top/cite/T-0001
```

## Windows

The `windows` snippet is a `.reg` registry export:

```reg
Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\Software\Classes\task]
@="URL:task Protocol"
"URL Protocol"=""
"FriendlyTypeName"="hop-top.myapp.go.task"
"HopHandlerID"="hop-top.myapp.go.task"

[HKEY_CURRENT_USER\Software\Classes\task\shell\open\command]
@="\"C:\\Program Files\\MyApp\\myapp.exe\" \"%1\""
```

### Install (per-user, no UAC)

```cmd
reg import myapp.reg
```

Or double-click the `.reg` file from Explorer; Windows prompts for
confirmation before merging into the registry.

### Install (system-wide)

Replace `HKEY_CURRENT_USER` with `HKEY_LOCAL_MACHINE` in the snippet and
import from an elevated shell. Per-user (HKCU) is what `cite` emits by
default because it doesn't require admin.

### Verify

```cmd
reg query HKCU\Software\Classes\task /ve
:: expected: (Default) REG_SZ URL:task Protocol

start task://hop-top/cite/T-0001
:: expected: myapp.exe launches with the URL as argv[1]
```

### Go shortcut

Go on Windows can write the same keys directly to HKCU without going
through a `.reg` file:

```go
err := handle.Register("task", `C:\Program Files\MyApp\myapp.exe`)
```

This is per-user and requires no elevation.

## Cross-cutting notes

### Per-language reach

| Platform | Runtime registration | Snippet generation |
| --- | --- | --- |
| Linux   | Go only (`handle.Register`) | all 5 SDKs |
| macOS   | Go only (CGO, LSSetDefault…) | all 5 SDKs |
| iOS     | not supported by any SDK (static-only) | all 5 SDKs |
| Windows | Go only (`handle.Register`) | all 5 SDKs |

TS/Python/Rust/PHP users always go through the snippet-then-install flow.
The artifact contract is identical regardless of which SDK emitted it, so
you can generate the `.desktop` in PHP and have a Go binary install it.

### Same scheme, multiple owners

`X-Hop-Handler-ID` (Linux), `CFBundleURLName` (macOS/iOS), and
`HopHandlerID` (Windows) carry the full `<vendor>.<app>.<language>.<scheme>`
identity in every artifact. Two different apps can install handlers for
the same scheme without filename collisions — but the OS still routes
each scheme to **one** default handler. Use `instance` in the spec
(e.g. `instance: "dev"`) to side-by-side install dev and prod variants.

Windows has an additional caveat: at the `HKCU\Software\Classes\<scheme>`
key level, only the most recent installer wins. See
[`handler-identity.md`](../specs/handler-identity.md) for the full policy.

### Validation

Linux `.desktop` files can be checked with `desktop-file-validate`. macOS
plists with `plutil -lint`. Windows `.reg` files have no first-party
validator, but `reg import` returns non-zero on syntax errors.

## See also

- [`handler-identity.md`](../specs/handler-identity.md) — the contract that
  makes `Handle` outputs identical across all 5 SDKs.
- [`cite-contract.md`](../specs/cite-contract.md) — the parser side of the
  story (what `task://hop-top/cite/T-0001` *means* once your scheme is
  registered).
- Per-language READMEs ([Go](../../go/README.md),
  [TypeScript](../../ts/README.md), [Python](../../py/README.md),
  [Rust](../../rs/README.md), [PHP](../../php/README.md)) — idiomatic
  usage examples.
