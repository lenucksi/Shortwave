---
id: SW-31
title: "Playback: Codec & Quality Preferences (SW-10)"
status: To Do
assignee: []
created_date: 2026-05-28 09:53
labels: []
dependencies: []
references:
  - src/ui/discovery_results_dialog.rs
  - src/settings/mod.rs
priority: high
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a settings dialog to let users set preferred codec (AAC > MP3 > Ogg) and minimum bitrate for stream selection in resolve_best_format.\n\nCurrently select_best_format() auto-picks the 'best' stream (TLS > non-TLS, AAC > MP3, higher bitrate). Users should be able to override this via settings.\n\nSettings should be stored in gsettings or a config file and applied during stream resolution (A1's fetch_and_parse / select_best_format pipeline).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Settings dialog with codec preference ordering (AAC, MP3, Ogg, Auto)
- [ ] #2 Settings dialog with minimum bitrate slider (e.g. 64-320 kbps)
- [ ] #3 Settings stored in gsettings (org.gnome.Shortwave schema)
- [ ] #4 select_best_format respects user preferences
- [ ] #5 Defaults to Auto (current behavior)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Build: ./tools/dev-build.sh gresource && ./tools/dev-build.sh binary
<!-- SECTION:PLAN:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
- [ ] #5 All AC items checked
- [ ] #6 cargo clippy --all -- -D warnings pass
- [ ] #7 Branch merged in main
<!-- DOD:END -->