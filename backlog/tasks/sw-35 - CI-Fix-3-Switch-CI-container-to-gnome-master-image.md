---
id: SW-35
title: "CI-Fix-3: Switch CI container to gnome-master image"
status: Done
assignee: []
created_date: 2026-05-28 14:58
updated_date: 2026-05-28 15:08
labels: []
dependencies: []
references:
  - https://github.com/lenucksi/Shortwave/actions/runs/26569090741/job/78271313947
  - https://github.com/lenucksi/Shortwave/actions/runs/26569090741/job/78271314084
modified_files:
  - .github/workflows/ci.yml
priority: high
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In build-test and lint jobs in ci.yml, change the container image from quay.io/gnome_infrastructure/gnome-runtime-images:gnome-50 to gnome-master to match the Flatpak manifest's runtime-version: master. This fixes 'org.gnome.Sdk/x86_64/master not installed' errors.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 build-test job completes without flatpak runtime errors
- [ ] #2 lint job completes without flatpak runtime errors
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read ci.yml\n2. Change image tag from gnome-50 to gnome-master in build-test job\n3. Change image tag from gnome-50 to gnome-master in lint job
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Changed container image from gnome-50 to gnome-master in both build-test and lint jobs to match Flatpak manifest's runtime-version: master. This fixes the 'org.gnome.Sdk/x86_64/master not installed' error.
<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->