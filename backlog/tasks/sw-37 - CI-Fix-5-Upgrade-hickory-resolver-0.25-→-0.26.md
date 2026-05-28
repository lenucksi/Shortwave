---
id: SW-37
title: "CI-Fix-5: Upgrade hickory-resolver 0.25 → 0.26"
status: Done
assignee: []
created_date: 2026-05-28 14:58
updated_date: 2026-05-28 15:08
labels: []
dependencies:
  - SW-36
references:
  - https://github.com/lenucksi/Shortwave/security/dependabot/15
  - https://github.com/lenucksi/Shortwave/security/dependabot/16
modified_files:
  - Cargo.toml
  - src/api/client.rs
priority: medium
ordinal: 31000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Upgrade hickory-resolver from 0.25 to 0.26 to fix RUSTSEC-2026-0118 (high) and RUSTSEC-2026-0119 (medium) vulnerabilities. Breaking change: TokioConnectionProvider renamed to TokioRuntimeProvider and moved from 'name_server' to 'net::runtime' module. This will also fix 2 open Dependabot alerts.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cargo build succeeds
- [ ] #2 RUSTSEC-2026-0118 and RUSTSEC-2026-0119 no longer trigger
- [ ] #3 Dependabot alerts 15 and 16 marked as fixed
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Bump hickory-resolver to 0.26 in Cargo.toml\n2. Change import TokioConnectionProvider → TokioRuntimeProvider\n3. Change module path name_server → net::runtime\n4. Remove advisory ignores from .deny.toml (only if upgrade fully resolves)\n5. cargo build / cargo test
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Upgraded hickory-resolver from 0.25 to 0.26. Changes: TokioConnectionProvider → TokioRuntimeProvider (net::runtime module), build() returns Result now, Lookup::into_iter() → Lookup::answers().iter(). hickory-proto 0.26.1 resolves both RUSTSEC-2026-0118 and RUSTSEC-2026-0119. Removed advisory ignores from .deny.toml.
<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->