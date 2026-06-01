# SW-40: WayDriver Proof-of-Concept Spike

## Status: ✅ Successful

Shortwave 5.1.0 boots successfully inside a headless Mutter session managed by
WayDriver. The AT-SPI tree confirms the main window, buttons, labels, and
sliders are all accessible. Screenshots render correctly.

## Docker Images

```bash
# Pull WayDriver base images (Fedora 42 based)
docker pull ghcr.io/bohdantkachenko/waydriver-mcp:latest
docker pull ghcr.io/bohdantkachenko/waydriver-mcp-builder:latest
```

## Architecture

We use **Fedora 43** as the base (not the WayDriver F42 images) because
Fedora 42 ships `libadwaita-1.7.12` but Shortwave 5.1 requires
`libadwaita >= 1.8` (uses `AdwDialog`, `AdwAlertDialog`, `AdwPreferencesDialog`).
Fedora 43 ships `libadwaita-1.8.5.1` and `glycin-2.0.8`.

The `waydriver-mcp` binary is copied from the official F42-based image; it's a
static-ish Rust binary and works fine on F43.

## Full System Dependencies (Fedora 43)

### Build-time (`dnf install`)

```bash
gcc g++ make pkg-config meson ninja-build cmake
dbus-devel at-spi2-core-devel
gstreamer1-devel gstreamer1-plugins-base-devel
gstreamer1-plugins-bad-free-devel
pipewire-devel
gtk4-devel glib2-devel libadwaita-devel
libshumate-devel libsoup3-devel
sqlite-devel openssl-devel
lcms2-devel libseccomp-devel
gettext-devel desktop-file-utils
glycin-devel glycin-gtk4-devel
git
# Rust (via rustup, not dnf):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

### Runtime (inside e2e container)

```bash
dbus dbus-x11 at-spi2-core
mutter pipewire wireplumber pipewire-gstreamer
gstreamer1 gstreamer1-plugins-base gstreamer1-plugins-good
gstreamer1-plugins-bad-free
gsettings-desktop-schemas
sqlite-libs openssl-libs lcms2 libseccomp
libshumate libsoup3
gtk4 libadwaita
glycin-gtk4-libs
google-noto-sans-vf-fonts google-noto-emoji-fonts
adwaita-icon-theme
python3
```

## Dockerfile

See `tmp/Dockerfile` in the worktree. Key stages:

1. **builder** (Fedora 43): installs build deps + Rust, runs `meson setup build
   -Dprofile=development`, `ninja -C build`, `ninja -C build install`
2. **runtime** (Fedora 43): WayDriver runtime deps + Shortwave binary + installed
   resources (gresource, schemas, desktop files, icons) + `waydriver-mcp` binary
   copied from the official F42 image.

## AT-SPI Tree (Key Nodes)

```xml
<Application role="application" name="shortwave" toolkit="GTK">
  <Window role="window" name="Shortwave" showing="true" visible="true"
          sensitive="true" active="true" bbox="0,0,975,675">
    <!-- Main content area -->
    <Group name="Welcome to Shortwave" />
    <!-- Player panel (right sidebar) -->
    <Label role="label" name="No Playback" />
    <Button role="button" name="Connect Device" />
    <Button role="button" name="Show Station Details" />
    <Group>
      <Button role="button" name="Toggle Mute" />
      <Slider role="slider" name="Volume" />
    </Group>
    <Separator role="separator" />
  </Window>
</Application>
```

Full dump: `tmp/waydriver-poc/atspi-dump.xml` (20KB, 113 nodes)

## Working XPath Selectors

| XPath | Matches | Notes |
|-------|---------|-------|
| `//Window` | 1 | `name='Shortwave'` |
| `//Window[@name='Shortwave']` | 1 | Canonical match |
| `//Button` | 3 | Connect Device, Show Station Details, Toggle Mute |
| `//Label` | 1 | "No Playback" |
| `//Slider` | 1 | Volume slider |
| `//Separator` | 1 | UI separator |

**Not working (0 matches):** `//MenuItem`, `//Entry`, `//List` — these widgets
aren't present in the welcome/default state.

## MCP API

waydriver-mcp v1.4.0 (protocol version 2024-11-05) exposes 30 tools via the
standard MCP `tools/call` interface:

```python
# Initialize
{"method": "initialize", "params": {"protocolVersion": "2024-11-05", ...}}
# List tools
{"method": "tools/list"}
# Call a tool
{"method": "tools/call", "params": {"name": "start_session", "arguments": {...}}}
```

Key tools used:
- `start_session` — boots headless Mutter, launches app
- `dump_tree` — returns AT-SPI XML tree
- `query` — evaluate XPath, returns JSON matches
- `take_screenshot` — capture PNG via PipeWire
- `kill_session` — tear down

## Screenshot

`tmp/waydriver-poc/screenshot-final.png` (197KB, 1280×720 render)

The screenshot shows the Shortwave welcome screen with:
- Left nav bar (3 generic icon slots)
- Main area: "Welcome to Shortwave" branding
- Right panel: album art placeholder, "No Playback" label,
  Connect Device / Show Station Details buttons, Volume slider

## Build Time

- First build (cold cargo cache): ~2m 40s
- Subsequent builds (cached): ~2s (Docker layer caching)

## Blocker: gresource Install

Shortwave loads its UI resources from a `.gresource` file at a path compiled
into the binary via the `MESON_DATADIR` env var. The initial Dockerfile only
copied the binary (`cp build/src/shortwave ...`) without running `ninja install`,
so the gresource file was missing and the app panicked:

```
Could not load resources: Failed to open file
"/usr/share/shortwave/de.haeckerfelix.Shortwave.Devel.gresource"
```

**Fix:** Run `DESTDIR=/install-staging ninja -C build install` in the builder
stage, then `COPY --from=builder /install-staging/usr/share/ /usr/share/` in the
runtime stage. GSettings schemas must be compiled with `glib-compile-schemas`.

## D-Bus Requirements

WayDriver's docker-entrypoint.sh pattern must be followed:
```bash
export XDG_RUNTIME_DIR=$(mktemp -d /tmp/waydriver-XXXXXX)
eval "$(dbus-launch --sh-syntax)"
```

Without private D-Bus, waydriver-mcp fails with: `D-Bus: I/O error: No such
file or directory (os error 2)`

## How to Run

```bash
# Build the Docker image
docker build -t shortwave-waydriver:latest \
  -f tmp/Dockerfile \
  --progress=plain .

# Run the full capture
docker run --rm \
  --security-opt seccomp=unconfined \
  --security-opt apparmor=unconfined \
  --cap-add=SYS_PTRACE \
  shortwave-waydriver:latest \
  sh -c '
    export HOME=/root
    export XDG_RUNTIME_DIR=$(mktemp -d /tmp/wd-XXXXXX)
    eval "$(dbus-launch --sh-syntax)"
    python3 /tmp/capture_artifacts.py
  '
```

## Next Steps for SW-41

1. The AT-SPI tree has many `Generic`/`Group`/`TabPanel` nodes with no
   `name` attribute — these are GTK4 container widgets that the accessibility
   bridge doesn't label. SW-41 should add accessibility labels/descriptions to
   key widgets (navigation buttons, search entry, station list, player controls).
2. The welcome page shows "Welcome to Shortwave" as a `Group` not a `Label` or
   `Heading` — the AT-SPI role hierarchy is generic. Consider using
   `gtk::Label` or `Adw::StatusPage` for better AT-SPI exposure.
3. XPath `//Button` returns 3 buttons but there are more visible in the
   screenshot (likely not widget::Button but event-controller-based clicks or
   `GtkGestureClick`). Need to check what the nav bar icons are.
