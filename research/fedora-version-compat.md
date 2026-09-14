# Fedora Base Version Compatibility for WayDriver

## 1. Fedora Version History — Key Package Versions

| Fedora | Release Date | glibc | GTK4 | libadwaita | GStreamer | PipeWire |
|--------|-------------|-------|------|-----------|-----------|---------|
| **42** | 2025-04-15 | 2.41 | 4.18.6 | 1.7.12 | 1.26.1 | 1.4.11 |
| **43** | 2025-10-28 | 2.42 | 4.20.4 | 1.8.5.1 | ~1.28 | 1.4.11 |
| **44** | 2026-04-28 | 2.43 | 4.22.4 | 1.9.0 | 1.28.2 | 1.6.6 |
| **45** (Rawhide) | ~2026-10 (beta Aug 2026) | 2.43.9000 | 4.23.0 | 1.9.1 | ~1.28/1.30 | 1.6.6 |

Sources:
- Fedora Packages: glibc, gtk4, libadwaita, pipewire-gstreamer, gstreamer1-plugin-gtk4
- Docker Hub `fedora` tags: 42, 43, 44, 45 (rawhide)

## 2. glibc Backward Compatibility Analysis

**The waydriver-mcp binary is a Rust binary compiled on Fedora 42 (glibc 2.41).**

### How glibc versioning works
- glibc uses **symbol versioning** — every exported symbol is tagged with the glibc version it was introduced in (e.g. `GLIBC_2.41`)
- A binary compiled on F42 records which `GLIBC_2.xx` symbols it needs
- **Newer glibc includes all older symbol versions** → a binary that needs `GLIBC_2.41` will run fine on a system with glibc 2.43
- glibc has maintained this backward compatibility across all `libc.so.6` versions (two decades)

### What waydriver-mcp needs
| Fedora (host) | glibc version | Has GLIBC_2.41? | Compatible? |
|---------------|--------------|-----------------|-------------|
| 42 (build) | 2.41 | Yes (native) | ✅ Reference |
| 43 | 2.42 | Yes (shipped) | ✅ |
| 44 | 2.43 | Yes (shipped) | ✅ |
| 45/Rawhide | 2.43.9000 | Yes (shipped) | ✅ |

**Verdict: glibc is not a blocker.** All supported Fedora versions ship glibc new enough to satisfy the F42-compiled binary.

### Risk of going too new
Rust's `std` links against glibc dynamically. If waydriver-mcp were ever *recompiled* on a very new Fedora, it could start requiring newer glibc symbols. But since we **copy the prebuilt binary from the F42 image**, the symbol requirements are frozen at glibc 2.41.

## 3. GTK4 ABI Compatibility

### waydriver-mcp binary
- waydriver-mcp is the MCP server — it does **not** directly link GTK4 or libadwaita
- It communicates with apps via AT-SPI (accessibility bus) and PipeWire (video capture)
- **No GTK4 ABI concern for waydriver-mcp itself**

### waydriver-fixture-gtk (e2e test binary)
- This binary *does* link GTK4 and libadwaita
- Built as part of the Docker build on F42 → linked against F42's libgtk-4.so.1 (4.18.6)
- GTK4 uses soname `libgtk-4.so.1` — **same soname across all 4.x releases**
- The GTK4 project committed to ABI stability within the 4.x series (post-4.6 stabilization)
- On F44, `libgtk-4.so.1` is provided by GTK 4.22.4 — runtime loading works fine
- libadwaita uses soname `libadwaita-1.so.0` — same stability guarantee

**Verdict: GTK4 ABI is safe across F42→F44/F45.** The soname is unchanged, and ABI is stable within 4.x.

## 4. GStreamer / PipeWire Compatibility

- waydriver-mcp uses `pipewiresrc` / `pipewiresink` GStreamer elements
- These are provided by the `pipewire-gstreamer` package (`libgstpipewire.so`)
- GStreamer core uses soname `libgstreamer-1.0.so.0` — stable across 1.x
- The PipeWire GStreamer plugin talks to the PipeWire daemon via the **PipeWire protocol**, not through the library ABI
- F42: GStreamer 1.26.1, PipeWire 1.4.11, pipewire-gstreamer 1.4.11
- F44: GStreamer 1.28.2, PipeWire 1.6.6, pipewire-gstreamer 1.6.6
- The GStreamer plugin ABI is stable within the 1.x major version

**Verdict: GStreamer/PipeWire backward compatible.** No ABI issues.

## 5. WayDriver Nix Flake — Separate Concern

The WayDriver Nix flake (`flake.nix`) tracks:
```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
};
```

This is **independent of Fedora**. The Nix flake is used for local development environments and pulls packages from nixpkgs, not from Fedora's repos. The flake.lock pins a specific nixpkgs revision.

The Docker image (which we actually use) is built from `Dockerfile` which uses `fedora:42`. The Nix flake is irrelevant to the Docker-based deployment.

## 6. Conclusion: Maximum Safe Fedora Base

| Fedora | WayDriver Compat? | Shortwave Compat? | Verdict |
|--------|------------------|------------------|---------|
| 42 | ✅ Native | ❌ libadwaita 1.7, glycin 1.x | Blocked |
| 43 | ✅ glibc BC, GTK4 soname OK | ✅ libadwaita 1.8.5 | **Safe** |
| 44 | ✅ glibc BC, GTK4 soname OK | ✅ libadwaita 1.9.0 | **Maximum safe stable** |
| 45/Rawhide | ✅ glibc BC, GTK4 soname OK | ✅ libadwaita 1.9.1 | Likely safe but pre-release |

**Maximum safe Fedora base: Fedora 44** (current latest stable release)

Rawhide (F45) is also likely compatible but:
- Pre-release software may have regressions
- Not recommended for reproducible CI builds
- The "safe" window closes once rawhide gets a new GTK major version (GTK5, libadwaita 2.x)

## 7. Recommendation for SW-42 Docker Image

**Target: Fedora 44**

Rationale:
1. **glibc 2.43** is backward-compatible with the F42-built waydriver-mcp binary — confirmed by glibc's symbol versioning guarantee
2. **GTK4 4.22.4** uses the same `libgtk-4.so.1` soname as F42's 4.18.6 — no ABI break
3. **libadwaita 1.9.0** satisfies Shortwave's `>= 1.8` requirement
4. **PipeWire 1.6.6** is backward-compatible with the PipeWire GStreamer plugin interface
5. Fedora 44 is the current stable release (as of May 2026), with active updates

### Docker strategy

```
# Build waydriver-mcp on Fedora 42 (upstream Dockerfile)
FROM fedora:42 AS waydriver-builder
COPY --from=ghcr.io/bohdantkachenko/waydriver-mcp:latest ...

# Runtime on Fedora 44
FROM fedora:44 AS runtime
COPY --from=waydriver-builder /usr/local/bin/waydriver-mcp /usr/local/bin/
RUN dnf install -y <shortwave-deps>
```

The prebuilt waydriver-mcp binary from the F42 CI is copied into an F44 runtime image. All runtime dependencies (libadwaita, glycin, GTK4, PipeWire, GStreamer) come from F44's repos.

### If Fedora 44 is unavailable

Fedora 43 is also fully compatible as a fallback — it has libadwaita 1.8.5 (>= 1.8) and glibc 2.42 (>= 2.41). Fedora 43 reaches EOL ~April 2026, so prefer 44 for longevity.

### Monitoring

Re-evaluate when:
- Fedora 45 stable is released (~October 2026) — should also be compatible
- WayDriver upstream bumps their Dockerfile past Fedora 42
- GTK5 or libadwaita 2.0 ships (major soname bump = ABI break)
