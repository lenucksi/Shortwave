---
id: SW-26
title: Cocogitto config + release workflow
status: To Do
assignee: []
created_date: 2026-05-26 13:32
labels: []
dependencies: []
modified_files:
  - cog.toml
  - .github/workflows/release.yml
priority: high
ordinal: 26000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up cocogitto for conventional commits + automated releases.

1. Create cog.toml:
   - Version: 5.1.0 (matching meson.build)
   - Configure commit types: feat, fix, docs, style, refactor, test, chore, ci, build, perf, revert
   - Configure changelog sections and ordering
   - Set up cog.toml for Rust project with appropriate pre-bump hooks
   - Ignore merge commits

2. Create .github/workflows/release.yml:
   - On push to main: cocogitto/cocogitto-action with release: true
   - cog bump --auto for automatic version bump based on conventional commits
   - Creates GitHub Release with changelog
   - git-user and git-user-email configured for bot identity

3. Add cocogitto-bot GitHub App to repo (descriptive note, done via GitHub UI):
   - Zero-config PR decoration with conventional commit status checks
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cog.toml created with version 5.1.0 and all required commit types
- [ ] #2 cog check passes on existing commits (using --from-latest-tag if needed)
- [ ] #3 Release workflow creates GitHub releases via cocogitto-action
- [ ] #4 All GitHub Actions SHA-pinned and verified
- [ ] #5 cargo fmt + cargo clippy pass
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
