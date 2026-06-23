# AccessKit + WayDriver: Compatibility Analysis

**Date:** 2026-06-01
**Context:** GTK 4.18 merged the AccessKit accessibility backend. What happens to WayDriver (AT-SPI-dependent) when `GTK_A11Y=accesskit` is set?

---

## 1. AccessKit Architecture

AccessKit (https://accesskit.dev/) is a **cross-platform, cross-language accessibility abstraction layer** written in Rust. Its design:

- **Data schema** (`accesskit` crate): Defines a tree of `Node`s with integer IDs, roles (Button, Label, etc.), attributes, and actions. Modeled after Chromium's cross-platform accessibility abstraction. Source: https://accesskit.dev/how-it-works

- **In-process, push-based**: The UI toolkit pushes tree updates to the platform adapter via normal function calls (no IPC). The platform adapter retains the complete tree. This is inspired by Chromium's multi-process architecture but both components run in the **same process**. Source: https://accesskit.dev/how-it-works

- **Platform adapters**: Separate crates that translate the AccessKit tree into native platform APIs:
  - `accesskit_unix` — implements **AT-SPI D-Bus interfaces** (using `zbus`, a pure-Rust D-Bus library)
  - `accesskit_windows` — implements UI Automation
  - `accesskit_macos` — implements NSAccessibility
  - `accesskit_android` / `accesskit_ios` — in development

  Source: https://github.com/AccessKit/accesskit (README), https://crates.io/crates/accesskit_unix

- **Consumer crate** (`accesskit_consumer`): Platform-independent tree-walking code, shared by platform adapters. Also useful for in-process assistive technologies. Source: https://docs.rs/accesskit_consumer

- **Key crates:**
  - `accesskit` (core schema, 3.1K SLoC) — https://crates.io/crates/accesskit
  - `accesskit_unix` (Linux/AT-SPI adapter) — https://crates.io/crates/accesskit_unix
  - `accesskit_atspi_common` (shared AT-SPI translation layer) — https://lib.rs/crates/accesskit_atspi_common
  - `accesskit_consumer` (tree walker/filter) — https://docs.rs/accesskit_consumer
  - `accesskit_winit` (winit integration)
  - `accesskit_python` (Python bindings via PyO3) — https://github.com/AccessKit/accesskit-python

- **No out-of-process protocol**: AccessKit has **no** D-Bus/remote query protocol of its own. On Linux, it uses AT-SPI D-Bus interfaces. On other platforms, it uses the native platform accessibility APIs.

---

## 2. AccessKit in GTK 4.18+

- **Merged in GTK 4.18**: The AccessKit backend was merged as an alternative accessibility backend. Build with `-Daccesskit=enabled`, then set `GTK_A11Y=accesskit`. Source: https://blogs.gnome.org/gtk/2025/05/12/an-accessibility-update/

- **Backend selection** (from GTK docs, https://docs.gtk.org/gtk4/running.html):
  - `GTK_A11Y=atspi` — AT-SPI backend (default on Linux)
  - `GTK_A11Y=accesskit` — AccessKit backend
  - `GTK_A11Y=test` — Test backend (recommended for CI/test suites)
  - `GTK_A11Y=none` — Disable accessibility entirely

  Multiple backends can be comma-separated and tried in order.

- **On Linux, AccessKit backend still speaks AT-SPI**: The `accesskit_unix` crate (used by GTK's AccessKit backend) implements the standard AT-SPI D-Bus interfaces (`org.a11y.atspi.*`). This means the accessibility tree is still queryable over the a11y D-Bus bus, just as with the native AT-SPI backend. Source: https://crates.io/crates/accesskit_unix ("It exposes an AccessKit accessibility tree through the AT-SPI protocol.")

- **AT-SPI is the default on Linux and not going away**: The GTK blog post explicitly states "The AccessKit backend works on Linux as well, but we are still defaulting to at-spi here." Source: https://blogs.gnome.org/gtk/2025/05/12/an-accessibility-update/

- **Why AccessKit matters on Linux**: It enables GTK accessibility on Windows and macOS for the first time. On Linux it provides an alternative implementation of the same AT-SPI protocol, with potential for the experimental "Newton" Wayland-native stack in the future.

---

## 3. AccessKit Client/Server Model

- **No standalone AccessKit client/consumer exists**: There is no `accesskit-client` binary, no D-Bus service for AccessKit, and no CLI tool to dump an AccessKit tree directly. The only way to query accessibility from outside the process on Linux is through AT-SPI D-Bus.

- **`accesskit_unix` exposes AT-SPI**: The adapter registers D-Bus objects on the a11y bus implementing `org.a11y.atspi.Accessible`, `org.a11y.atspi.Action`, `org.a11y.atspi.Component`, `org.a11y.atspi.Text`, `org.a11y.atspi.EditableText`, `org.a11y.atspi.Selection`, etc. Any AT-SPI client (including WayDriver) can connect and interact as normal.

- **`accesskit_consumer` is in-process only**: The `accesskit_consumer` crate provides tree-walking and filtering APIs, but it requires direct access to the `accesskit` tree data — it's designed for in-process use (e.g., an embedded screen reader), not for remote query.

- **The "Newton" project**: An experimental GNOME accessibility stack that pushes AccessKit tree updates directly to assistive technologies over a new Wayland protocol. Mentioned in `accesskit_atspi_common` crate docs: "shared by the AccessKit Unix adapter and the new, experimental GNOME accessibility stack (code-named Newton)." This is **not yet production-ready** and would require support in compositors, toolkits, and screen readers. Source: https://blogs.gnome.org/tbernard/2025/04/ (STF update), https://lib.rs/crates/accesskit_atspi_common

- **Python bindings**: `accesskit-python` exists but is for toolkits to *implement* accessibility, not for external query.

---

## 4. WayDriver's AT-SPI Dependency

WayDriver's accessibility client lives in `crates/waydriver/src/atspi.rs` and is deeply coupled to AT-SPI D-Bus:

- **Connection**: Connects to the a11y bus via `atspi` + `zbus` Rust crates, gets registry root at `org.a11y.atspi.Registry:/org/a11y/atspi/accessible/root`
- **Tree snapshot**: Walks the D-Bus object tree using `AccessibleProxy::get_children()`, calling `get_role_name()`, `name()`, `get_state()`, `get_attributes()`, `get_extents()` per node, serializing to XML
- **XPath queries**: Runs XPath 1.0 over the serialized XML using `sxd-xpath`
- **Actions**: Invokes AT-SPI interfaces directly: `ActionProxy::do_action(0)`, `ComponentProxy::grab_focus()`, `EditableTextProxy::set_text_contents()`, `TextProxy::get_text()`, `SelectionProxy::select_child()`
- **Key deps**: `atspi`, `zbus`, `sxd-document`, `sxd-xpath`

The three traits (`CompositorRuntime`, `InputBackend`, `CaptureBackend`) do **not** cover accessibility. The AT-SPI client is baked directly into the `Session` struct and `Locator` API — there's no `AccessibilityBackend` trait.

Source: https://github.com/BohdanTkachenko/waydriver (raw source analysis of `crates/waydriver/src/lib.rs`, `atspi.rs`, `Cargo.toml`)

---

## 5. State of AccessKit Tooling

- **Elevado**: A new Rust AT-SPI inspector (replacement for accerciser), launched at https://gitlab.gnome.org/feaneron/elevado/. Still an AT-SPI client — connects to the a11y D-Bus bus. Not AccessKit-specific. Source: https://blogs.gnome.org/gtk/2025/05/12/an-accessibility-update/

- **No AccessKit-specific tooling exists**: No `accesskit-dump-tree`, no `accesskit-inspect`, no AccessKit-native CLI or GUI explorer.

- **Chrome's `ax_dump_tree`**: Chromium has `ax_dump_tree` and `ax_dump_events` CLI tools (https://chromium.googlesource.com/chromium/src/+/refs/tags/126.0.6441.0/tools/accessibility/inspect/README.md) but these work with Chrome's own accessibility tree, not generic AccessKit providers.

- **Testing recommendations**: The GTK docs recommend `GTK_A11Y=test` for test suites and CI pipelines (https://docs.gtk.org/gtk4/running.html), not for providing an inspectable tree. WayDriver's use of AT-SPI is the correct approach for real UI testing.

---

## 6. Practical Impact for Shortwave

### Current setup
Shortwave sets `GTK_A11Y=atspi` in the Docker entrypoint. The app runs in a headless Mutter session with a private D-Bus bus, and WayDriver connects to the a11y bus to inspect/interact.

### Future scenarios

| Scenario | Breaks? | Reason |
|---|---|---|
| `GTK_A11Y=atspi` (current, default) | No | AT-SPI backend, WayDriver works |
| `GTK_A11Y=accesskit` | **No** | `accesskit_unix` still exposes AT-SPI D-Bus interfaces; WayDriver connects and works identically |
| `GTK_A11Y=none` | Yes | No accessibility tree at all |
| `GTK_A11Y=test` | Yes | Test backend doesn't register on the a11y bus; designed for in-process unit testing |
| Future "Newton" protocol (experimental) | **Maybe** | If AT-SPI is bypassed entirely, WayDriver would need a new backend. But this is years away and would need compositor, toolkit, and screen reader buy-in. |

### Risk assessment
- **Low risk**: AT-SPI is the default on Linux and the GTK blog explicitly states this isn't changing. The AccessKit backend on Linux speaks AT-SPI too.
- **Medium-long term**: If the experimental "Newton" Wayland-native accessibility protocol replaces AT-SPI on Linux, WayDriver would need an `AccessibilityBackend` trait and a Newton backend. But this is speculative and years out.
- **No action needed now**: WayDriver works correctly with both AT-SPI and AccessKit backends on Linux because both expose the same AT-SPI D-Bus protocol.

### If WayDriver wanted to support AccessKit directly (out-of-process)
Currently **impossible** — there is no out-of-process AccessKit query protocol. Options:
1. Wait for a future AccessKit remote protocol (not planned AFAIK)
2. Use `accesskit_consumer` in-process (would require running inside the target app — defeats the purpose)
3. Build WayDriver as an AT-SPI consumer (already done, works regardless of backend)
4. Contribute to / prepare for the "Newton" protocol (long-term)

---

## Key Sources

1. AccessKit "How it works" — https://accesskit.dev/how-it-works
2. GTK blog "An accessibility update" (2025-05-12) — https://blogs.gnome.org/gtk/2025/05/12/an-accessibility-update/
3. GTK 4.0 running/debugging docs (GTK_A11Y) — https://docs.gtk.org/gtk4/running.html
4. accesskit_unix crate — https://crates.io/crates/accesskit_unix
5. accesskit_atspi_common crate — https://lib.rs/crates/accesskit_atspi_common
6. WayDriver source (atspi.rs, lib.rs, Cargo.toml) — https://github.com/BohdanTkachenko/waydriver
7. GNOME STF update (Newton project) — https://blogs.gnome.org/tbernard/2025/04/
8. accesskit_consumer docs — https://docs.rs/accesskit_consumer
9. GTK 4 accessibility docs — https://docs.gtk.org/gtk4/section-accessibility.html
