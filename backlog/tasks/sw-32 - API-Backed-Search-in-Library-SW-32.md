---
id: SW-32
title: API-Backed Search in Library (SW-32)
status: To Do
assignee: []
created_date: 2026-05-28 09:53
labels: []
dependencies: []
references:
  - src/ui/pages/library_page.rs
  - data/gtk/library_page.ui
priority: medium
ordinal: 10100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a search bar to the library page that queries Radio-Browser.info for stations matching a search term. Results appear in the existing station grid alongside local library stations, with clear indication of which results are remote.\n\nCurrently the library only shows locally imported stations. Users must discover stations via the Discovery dialog (which scans all providers). An inline search bar would allow quick lookups against the Radio-Browser API without opening a separate dialog.\n\nSearch should:\n- Debounce input (300ms)\n- Query Radio-Browser.info /json/stations/byname/{query}\n- Show results inline in the library grid\n- Mark remote results with a badge/icon\n- Allow one-click import of remote results into library
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Search bar at top of library page
- [ ] #2 Debounced (300ms) API calls to Radio-Browser.info
- [ ] #3 Results shown inline in station grid
- [ ] #4 Remote results visually distinguished from local stations
- [ ] #5 One-click import of remote stations into library
- [ ] #6 Loading indicator during API call
- [ ] #7 Error handling for network failures
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