---
id: doc-6
title: "CI/CD Build Process: GitHub Actions Workflows"
type: guide
created_date: 2026-05-28 20:57
updated_date: 2026-05-28 21:50
labels:
  - ci
  - flatpak
  - build
  - infrastructure
---
# CI/CD Build Process: GitHub Actions Workflows

## Overview

Shortwave uses two GitHub Actions workflows for CI/CD:

- **`ci.yml`** — Code quality validation (build, test, lint, security audit)
- **`flatpak.yml`** — Flatpak artifact packaging (bundle, signing, deployment)

## Container Image

Both workflows use the same base container:

```
ghcr.io/flathub-infra/flatpak-github-actions:gnome-50
```

This image provides:
- Fedora-based OS with flatpak 1.15.x + flatpak-builder 1.4.x
- **Pre-installed** GNOME 50 Platform + SDK (`org.gnome.Platform//50`, `org.gnome.Sdk//50`)
- `flatpak-builder-lint` 3.0.0
- Python 3.12 + pip

**No remote-add or runtime install steps needed** — the GNOME SDK is already in the image.

Available tags: `gnome-48`, `gnome-49`, `gnome-50`, `freedesktop-24.08`, `freedesktop-25.08`, `kde-6.9`, `kde-6.10` (see [actions-images](https://github.com/flathub-infra/actions-images)).

Both containers require `options: --privileged` because bubblewrap needs user namespace creation, which Docker blocks by default via seccomp.

## ci.yml — CI / Validation

### Jobs

| Job | Container | Purpose |
|-----|-----------|---------|
| `prek` | none (ubuntu-latest) | Run pre-commit hooks (formatting, linting) |
| `conventional-commits` | none (ubuntu-latest) | Check commit messages against Conventional Commits spec via cocogitto |
| `build-test` | ghcr:gnome-50 (privileged) | Compile Rust code + run unit tests inside Flatpak sandbox |
| `lint` | ghcr:gnome-50 (privileged) | Run `cargo fmt`, `cargo clippy`, plus `flatpak-builder-lint` |
| `deny` | none (ubuntu-latest) | Audit dependencies via `cargo-deny` (advisories + licenses) |

### Build Process (build-test + lint)

Both jobs follow a three-step pattern inside the Flatpak sandbox:

#### Step 1: Bootstrap Flatpak SDK

Builds all dependency modules (currently only `libshumate`, a C map widget library) and stops before the `shortwave` module:

```bash
flatpak-builder \
  --disable-rofiles-fuse --keep-build-dirs \
  --state-dir=.flatpak-builder \
  --install-deps-from=flathub \
  --stop-at=shortwave \
  flatpak_app \
  build-aux/de.haeckerfelix.Shortwave.Devel.json

flatpak build-finish --share=network flatpak_app
```

- `--install-deps-from=flathub`: Auto-installs SDK extensions (rust-stable, llvm21) from Flathub
- `--state-dir=.flatpak-builder`: Ensures state is on the same filesystem as the build dir (required for container mounts)
- `flatpak build-finish --share=network`: Marks the build dir to allow network access (for cargo crate downloads)

#### Step 2: Build/Test/Lint inside Flatpak shell

Commands run inside the sandbox with access to the GNOME SDK + build dependencies:

```bash
echo "cd .. && cargo build" | \
  flatpak-builder --disable-rofiles-fuse \
    --state-dir=.flatpak-builder \
    --build-shell=shortwave \
    flatpak_app \
    build-aux/de.haeckerfelix.Shortwave.Devel.json
```

This compiles against the **exact same GNOME runtime** as the deployed Flatpak, ensuring ABI compatibility.

### Caching

`.flatpak-builder/` is cached via `actions/cache@v5` with key:
```
flatpak-builder-${arch}-${hashFiles('build-aux/de.haeckerfelix.Shortwave.Devel.json')}
```

This caches:
- Built `libshumate` module (avoids rebuild)
- Downloaded git sources
- Rust `target/` directory from inside the sandbox
- ccache data

### Flatpak Lint (new)

The lint job runs `flatpak-builder-lint` on the manifest and repo:

```bash
flatpak-builder-lint manifest build-aux/de.haeckerfelix.Shortwave.Devel.json
```

Expected warnings (non-blocking for development builds):
- `module-libshumate-source-git-branch`: Uses a git branch instead of pinned commit
- `appstream-screenshots-not-mirrored-in-ostree`: External screenshot URLs

### Why not compile outside the Flatpak?

Shortwave requires **libadwaita >= 1.7** (via `features = ["v1_7"]` in Cargo.toml). Ubuntu 24.04 LTS only provides libadwaita 1.5.0. The Flatpak GNOME 50 SDK provides libadwaita 1.9+, making it the only viable build environment.

## flatpak.yml — Flatpak Packaging

### Job: package-flatpak

Uses the `flatpak/flatpak-github-actions/flatpak-builder@v6` GitHub Action, which is a JavaScript wrapper that:
1. Runs `flatpak-builder` to completion (all modules including `shortwave`)
2. Exports the build to a local Flatpak repository
3. Creates a `.flatpak` single-file bundle
4. Handles caching automatically

The action runs **inside** the ghcr:gnome-50 container, using the pre-installed GNOME SDK.

### Artifacts

| Artifact | Location | Size | Retention |
|----------|----------|------|-----------|
| `.flatpak` bundle | `Shortwave-{version}.flatpak` | ~98 MB | 30 days |
| Flatpak repository | `repo/` | ~124 MB | 30 days |

The Flatpak repository can be hosted on GitHub Pages for OTA distribution (`index.flatpakrepo` generation included).

### Additional Jobs

- `deploy-flatpak-pages`: Deploys the repository to GitHub Pages (on release only)
- `upload-to-release`: Uploads the `.flatpak` bundle to the GitHub Release (on release only)

## Complete Build Flow

```
┌─────────────────────────────────────────────────────┐
│ Source Commit (push to main/master)                  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─ ci.yml ────────────────────────────────────┐    │
│  │ prek → conventional-commits → build-test →  │    │
│  │ → lint → deny                               │    │
│  └─────────────────────────────────────────────┘    │
│                                                     │
│  ┌─ flatpak.yml ───────────────────────────────┐    │
│  │ package-flatpak → (deploy/upload on release)│    │
│  └─────────────────────────────────────────────┘    │
│                                                     │
│  Both workflows run in parallel.                    │
│  ci.yml gates quality; flatpak.yml gates delivery.  │
└─────────────────────────────────────────────────────┘
```

## What Was Fixed

- **Container image**: Changed from `quay.io/gnome_infrastructure/gnome-runtime-images:gnome-master` to `ghcr.io/flathub-infra/flatpak-github-actions:gnome-50`
- **Container options**: Added `--privileged` (needed for bubblewrap)
- **Flatpak mode**: Switched from `--user` to system-wide (ghcr default)
- **Runtime installation**: Eliminated — GNOME SDK is pre-installed in the ghcr image
- **SDK extensions**: Auto-installed via `--install-deps-from=flathub` (no manual remote-add needed)
- **State directory**: Explicitly set via `--state-dir=.flatpak-builder` (avoids cross-filesystem issues)
- **Cargo fmt**: Now runs inside the Flatpak sandbox (not outside, where `cargo` wasn't available)
- **Caching**: Added `actions/cache` for `.flatpak-builder/` to avoid rebuilding `libshumate`
- **Flatpak lint**: Added `flatpak-builder-lint` to the lint job