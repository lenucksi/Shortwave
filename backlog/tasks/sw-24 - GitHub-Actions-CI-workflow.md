---
id: SW-24
title: GitHub Actions CI workflow
status: To Do
assignee: []
created_date: 2026-05-26 13:32
labels: []
dependencies: []
modified_files:
  - .github/workflows/ci.yml
priority: high
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create .github/workflows/ci.yml replacing GitLab CI checks.

Triggers: push + pull_request on main/master.

Jobs:
1. **prek checks**: use j178/prek-action to run fmt, clippy, typos hooks
2. **cocogitto check**: use cocogitto/cocogitto-action with command: check
3. **build + test**: cargo build + cargo test (with matrix if applicable)
4. **cargo deny**: license + advisory checks

All actions must be SHA-pinned (verified via git ls-remote before commit).

Replaces .gitlab-ci.yml equivalents: cargo-fmt, cargo-typos, cargo-deny, cargo-clippy, potfiles.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Workflow triggers on push and pull_request to main/master
- [ ] #2 prek hooks run via j178/prek-action covering format, lint, typos
- [ ] #3 Conventional commits checked via cocogitto/cocogitto-action
- [ ] #4 cargo build + cargo test succeed
- [ ] #5 cargo deny runs for license and advisory checks
- [ ] #6 All action references pinned to verified full commit SHAs
- [ ] #7 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
