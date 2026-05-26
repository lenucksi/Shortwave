---
id: SW-25
title: Flatpak build + repo deployment
status: To Do
assignee: []
created_date: 2026-05-26 13:32
labels: []
dependencies: []
references:
  - /home/jo/kit/sieve/SieveEditor/.github/workflows/package.yml
modified_files:
  - .github/workflows/flatpak.yml
priority: high
ordinal: 25000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create .github/workflows/flatpak.yml for automated Flatpak builds.

Triggers:
- Push to main (nightly builds)
- Release published (stable builds)
- workflow_dispatch (manual)

Jobs:
1. **package-flatpak**: Build Flatpak from build-aux/de.haeckerfelix.Shortwave.Devel.json
   - Use ghcr.io/flathub-infra/flatpak-github-actions:gnome-master container --privileged
   - Import GPG key via crazy-max/ghaction-import-gpg (secrets: GPG_PRIVATE_KEY, GPG_PASSPHRASE)
   - Use flatpak/flatpak-github-actions/flatpak-builder
   - Generate Flatpak repo with flatpak build-update-repo --generate-static-deltas
   - Upload .flatpak bundle and repo/ as artifacts
   - Attest build provenance via actions/attest-build-provenance

2. **deploy-flatpak-pages**: Deploy Flatpak repo to GitHub Pages
   - Download repo/ artifact
   - Generate index.flatpakrepo pointing to https://lenucksi.github.io/Shortwave/
   - Use actions/upload-pages-artifact + actions/deploy-pages
   - Only on release or tag

Pattern from SieveEditor's package.yml flatpak job + deploy-flatpak-pages.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Flatpak builds on push to main, release published, and manual dispatch
- [ ] #2 GPG signing works with GPG_PRIVATE_KEY + GPG_PASSPHRASE secrets
- [ ] #3 .flatpak bundle uploaded as artifact
- [ ] #4 Flatpak repo generated with static deltas and uploaded as artifact
- [ ] #5 index.flatpakrepo generated with correct URL and GPG key
- [ ] #6 Flatpak repo deployed to GitHub Pages on release
- [ ] #7 All actions SHA-pinned and verified
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
