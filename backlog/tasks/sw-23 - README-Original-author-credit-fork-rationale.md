---
id: SW-23
title: "README: Original author credit + fork rationale"
status: To Do
assignee: []
created_date: 2026-05-26 13:32
labels: []
dependencies: []
modified_files:
  - README.md
priority: high
ordinal: 23000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update README.md to:
- Credit Felix Häcker (haeckerfelix) as original author of Shortwave
- Explain the fork reason: unable to fork on gitlab.gnome.org, so published to GitHub
- Add pointer to upstream repo: https://gitlab.gnome.org/World/Shortwave
- Keep existing content (FAQ, building, translations, CoC)
- Add a "Fork Notes" or "Origin" section near the top
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Felix Häcker credited as original author with link to his GNOME profile
- [ ] #2 Fork reason explained clearly and neutrally
- [ ] #3 Upstream repository URL prominently linked
- [ ] #4 Existing README content preserved intact
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
