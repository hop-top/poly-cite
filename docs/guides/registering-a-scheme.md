---
doc_type: kb
subtype: guide
status: draft
title: Make a custom URI scheme open your app
summary: Take a cite-generated handler artifact and register it with Linux, macOS, iOS, or Windows so `<scheme>://...` URLs route to your application.
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

# Make a custom URI scheme open your app

After this guide, clicking `task://hop-top/cite/T-0001` in a browser, terminal, or
chat client will launch your application with the URL as input — on Linux, macOS,
iOS, or Windows.

## Use this when

- You ship an app that wants to own a custom scheme (`task://`, `tlc://`, `myapp://`).
- You package an app that already owns a scheme and need an install recipe per OS.
- You set up a workstation that should route someone else's scheme to a known app.

## Pick your starting point

| You are… | Start at |
| --- | --- |
| **App author** wiring scheme support into your codebase | [Step 1: build a HandlerSpec](#step-1--build-a-handlerspec) |
| **Package maintainer** writing an install recipe | [Step 3](#step-3--install-on-the-target-os) — start at your OS |
| **End user / sysadmin** installing someone else's pre-generated artifact | [Step 3](#step-3--install-on-the-target-os) — start at your OS |
| **cite contributor** changing the contract | [`docs/specs/handler-identity.md`](../specs/handler-identity.md) |

## Before you begin

You need:

- A scheme name (`task`, `tlc`, etc.) — must not collide with reserved URI schemes.
- A path to your executable (`/usr/bin/myapp`, `C:\Program Files\MyApp\myapp.exe`) — or, on macOS, a bundle identifier.
- `cite` installed in your language of choice (`go get hop.top/cite`, `pnpm add @hop-top/cite`, `pip install hop-top-cite`, `cargo add hop-top-cite`, `composer require hop-top/cite`).

## Result

A working `<scheme>://` URL. Clicking, `open`, `xdg-open`, or `start` routes to
your app with the full URL passed as argv.

---

## Step 1 — Build a HandlerSpec

`HandlerSpec` is the same shape in every SDK. Required fields:

| Field | Example | Notes |
| --- | --- | --- |
| `vendor` | `"hop-top"` | Your org/namespace |
| `app` | `"myapp"` | Your application name |
| `language` | `"go"` / `"ts"` / `"py"` / `"rs"` / `"php"` | Which SDK is emitting the artifact |
| `scheme` | `"task"` | The URI scheme |
| `appPath` | `/usr/bin/myapp` | Linux/Windows: executable path. macOS: bundle ID. |

Optional: `displayName`, `instance` (for side-by-side dev/prod installs), `version`, `channel`.

Why `language` is required: when two SDKs (e.g. a Go binary and a PHP CLI) both
own the same scheme, the `language` field keeps their artifact filenames from
colliding. See [`handler-identity.md`](../specs/handler-identity.md).

## Step 2 — Generate the snippet

All 5 SDKs expose `snippet(platform, spec)`. Platforms: `"linux"`, `"macos"`, `"ios"`, `"windows"`.

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

## Step 3 — Install on the target OS

Pick your OS:

- [Linux](#linux)
- [macOS](#macos)
- [iOS](#ios)
- [Windows](#windows)

---

## Linux

### Quick path

```sh
mkdir -p ~/.local/share/applications
printf '%s' "$out" > ~/.local/share/applications/hop-top.myapp.go.task.desktop
update-desktop-database ~/.local/share/applications
xdg-mime default hop-top.myapp.go.task.desktop x-scheme-handler/task
```

The filename must equal `<handlerID>.desktop`. Get it from
`desktop_filename(spec)` / `Handle::desktopFilename()` / equivalent.

### What the snippet looks like

```ini
[Desktop Entry]
Type=Application
Name=hop-top.myapp.go.task
Exec=/usr/bin/myapp %u
MimeType=x-scheme-handler/task;
NoDisplay=true
X-Hop-Handler-ID=hop-top.myapp.go.task
```

### Verify

```sh
xdg-mime query default x-scheme-handler/task
# expected: hop-top.myapp.go.task.desktop

xdg-open task://hop-top/cite/T-0001
# expected: /usr/bin/myapp launches with the URL as argv[1]
```

### System-wide install

```sh
sudo cp hop-top.myapp.go.task.desktop /usr/share/applications/
sudo update-desktop-database /usr/share/applications
```

Then any user on the system can run `xdg-mime default …` to opt in.

### Troubleshooting

| Symptom | Cause | Fix |
| --- | --- | --- |
| `xdg-mime query` returns empty | `update-desktop-database` not run, or file in wrong dir | Re-run install, confirm `~/.local/share/applications/` exists |
| `xdg-open` shows "file not found" | `Exec=` path is wrong | Edit `.desktop`, point at the correct binary |
| Browser ignores the click | `desktop-file-validate` errors | Run `desktop-file-validate <file>.desktop`, fix reported issues |
| Wrong app opens | Another `.desktop` is set as default | `xdg-mime default <yours>.desktop x-scheme-handler/<scheme>` again |

### Go shortcut

If your app is written in Go, skip steps 2–3 entirely:

```go
import "hop.top/cite/handle"

err := handle.Register("task", "/usr/bin/myapp")
```

Writes the `.desktop` file and calls `xdg-mime` in one go. No CGO.

---

## macOS

### Quick path

1. Generate the snippet with `platform: "macos"`.
2. Merge into your `.app`'s `Contents/Info.plist`. The SDKs ship a `patchPlist` /
   `patch_plist` / `PatchPlist` helper that does this:

   ```go
   out, err := generate.PatchPlist(srcInfoPlistReader, spec)
   ```

3. Ship the updated bundle. Launch Services registers it on first run, or
   manually:

   ```sh
   /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister \
       -f /Applications/MyApp.app
   ```

### What the snippet looks like

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

### Verify

```sh
plutil -lint /Applications/MyApp.app/Contents/Info.plist
# expected: OK

open task://hop-top/cite/T-0001
# expected: MyApp opens with the URL
```

### Troubleshooting

| Symptom | Cause | Fix |
| --- | --- | --- |
| `plutil -lint` reports invalid XML | Merge inserted the snippet in the wrong spot | Use `PatchPlist` instead of hand-merging |
| `open` opens nothing | Bundle not registered with Launch Services | Run `lsregister -f /Applications/MyApp.app` |
| Wrong app opens | Another bundle declared the same scheme first | Call `handle.Register(scheme, bundleID)` (Go) to force yours as default |

### Go shortcut

```go
err := handle.Register("task", "com.hop-top.myapp")  // bundle ID, not path
```

Sets the user's default. The bundle's `Info.plist` still must declare the
scheme — runtime register just picks the winner when multiple bundles declare it.

---

## iOS

### Quick path

1. Generate the snippet with `platform: "ios"`.
2. Embed in your app's `Info.plist` **at build time**.
   - Xcode: paste into the `Info` tab → URL Types, or edit `Info.plist` source.
   - gomobile / Fastlane: `PatchPlist` before code-signing.
3. Ship via TestFlight or the App Store.

iOS has no runtime registration API. The snippet must be in the bundle before
signing.

### What the snippet looks like

Identical to macOS — same `CFBundleURLTypes` fragment.

### Verify (Simulator)

```sh
xcrun simctl openurl booted task://hop-top/cite/T-0001
# expected: MyApp opens in the booted simulator
```

On real devices, test by tapping a link in Notes or Safari.

### Troubleshooting

| Symptom | Cause | Fix |
| --- | --- | --- |
| Build rejects the plist | XML invalid after merge | Use `PatchPlist`; don't hand-edit |
| `xcrun simctl openurl` silently no-ops | App not installed in simulator | `xcrun simctl install booted MyApp.app` first |
| Two apps declare the same scheme on device | iOS routes nondeterministically | Use distinct scheme names; iOS provides no tiebreaker |

---

## Windows

### Quick path

```cmd
:: snippet content saved to myapp.reg
reg import myapp.reg
```

Or double-click the `.reg` file in Explorer. Per-user (`HKCU`); no UAC prompt.

### What the snippet looks like

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

### Verify

```cmd
reg query HKCU\Software\Classes\task /ve
:: expected: (Default) REG_SZ URL:task Protocol

start task://hop-top/cite/T-0001
:: expected: myapp.exe launches with the URL as argv[1]
```

### System-wide install

Replace `HKEY_CURRENT_USER` with `HKEY_LOCAL_MACHINE` in the snippet and
`reg import` from an elevated shell. Per-user is the default because it avoids
UAC.

### Troubleshooting

| Symptom | Cause | Fix |
| --- | --- | --- |
| `reg import` returns non-zero | `.reg` syntax error (often a stray backslash) | Re-generate from `snippet()`; don't hand-edit |
| `start task://...` opens a "How do you want to open this?" dialog | Multiple apps registered for the scheme | Choose yours and check "Always use" |
| App opens but URL is empty | Command string missing `"%1"` | Re-generate; the snippet always quotes `%1` correctly |

### Go shortcut

```go
err := handle.Register("task", `C:\Program Files\MyApp\myapp.exe`)
```

Writes the same `HKCU` keys directly. Per-user, no UAC.

---

## How it works

`Handle` separates **artifact identity** from **OS routing key**:

| OS | Routes by | Artifact carries identity in |
| --- | --- | --- |
| Linux | `.desktop` filename + `MimeType=` | `X-Hop-Handler-ID=` |
| macOS / iOS | `CFBundleURLSchemes` | `CFBundleURLName` |
| Windows | `HKCU\Software\Classes\<scheme>` | `HopHandlerID` value |

Identity = `<vendor>.<app>[.<instance>].<language>.<scheme>`. Two SDKs can ship
artifacts for the same scheme without filename collisions, but the OS still picks
**one** default handler per scheme. Use `instance` to side-by-side install (e.g.
`instance: "dev"` for a dev build alongside prod).

Spec: [`docs/specs/handler-identity.md`](../specs/handler-identity.md).

## Runtime register vs. snippet — coverage

| Platform | `handle.Register()` (runtime) | `snippet()` (artifact) |
| --- | --- | --- |
| Linux | Go only | Go, TS, Python, Rust, PHP |
| macOS | Go only (CGO) | Go, TS, Python, Rust, PHP |
| iOS | not supported (no API) | Go, TS, Python, Rust, PHP |
| Windows | Go only | Go, TS, Python, Rust, PHP |

The artifact format is identical across SDKs — generate a `.desktop` in PHP,
ship it inside a Go binary, install it from a shell script.

## Next steps

- Parse the URLs your handler receives: [`docs/specs/cite-contract.md`](../specs/cite-contract.md).
- Add vanity aliases or action routing: per-language READMEs ([Go](../../go/README.md), [TypeScript](../../ts/README.md), [Python](../../py/README.md), [Rust](../../rs/README.md), [PHP](../../php/README.md)).
- Side-by-side dev/prod installs: set `instance` on the spec, re-generate.
