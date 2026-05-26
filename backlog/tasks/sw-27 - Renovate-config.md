---
id: SW-27
title: Renovate config
status: To Do
assignee: []
created_date: 2026-05-26 13:32
labels: []
dependencies: []
modified_files:
  - renovate.json
priority: medium
ordinal: 27000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create renovate.json for automated dependency updates.

Pattern adapted from SieveEditor's renovate.json, tailored for Rust/Cargo.

Configuration:
- extends: config:best-practices, helpers:pinGitHubActionDigests, :dependencyDashboard, :semanticCommits
- packageRules for Rust/Cargo (cargo) and GitHub Actions (github-actions)
- Weekly schedule (0 4 * * *), Europe/Berlin timezone
- Auto-merge for minor/patch/pin/digest updates
- vulnerabilityAlerts with security label
- Conventional commit format: chore(deps):
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 renovate.json created with Rust Cargo and GitHub Actions managers
- [ ] #2 Weekly schedule configured (0 4 * * *)
- [ ] #3 Auto-merge enabled for minor/patch updates
- [ ] #4 Dependency dashboard enabled
- [ ] #5 Semantic commit messages configured
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
