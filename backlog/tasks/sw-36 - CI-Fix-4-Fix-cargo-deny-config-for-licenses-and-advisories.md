---
id: SW-36
title: "CI-Fix-4: Fix cargo-deny config for licenses and advisories"
status: Done
assignee: []
created_date: 2026-05-28 14:58
updated_date: 2026-05-28 15:08
labels: []
dependencies: []
references:
  - https://github.com/lenucksi/Shortwave/actions/runs/26569090741/job/78271313922
modified_files:
  - .deny.toml
priority: high
ordinal: 30000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Fix .deny.toml: add CC0-1.0 license to allow list (for tiny-keccak crate), add RUSTSEC-2026-0118 and RUSTSEC-2026-0119 to ignore list (hickory-proto vulnerabilities until upgrade is possible).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cargo deny check advisories licenses passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read current .deny.toml\n2. Add CC0-1.0 to licenses.allow\n3. Add RUSTSEC-2026-0118 and RUSTSEC-2026-0119 to advisories.ignore
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added CC0-1.0 to allowed licenses (for tiny-keccak). Added RUSTSEC-2026-0118 and RUSTSEC-2026-0119 to advisory ignores (for hickory-proto until upgrade).
<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->