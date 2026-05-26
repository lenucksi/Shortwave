---
id: SW-29
title: Actionlint workflow
status: To Do
assignee: []
created_date: 2026-05-26 13:33
labels: []
dependencies: []
modified_files:
  - .github/workflows/actionlint.yml
priority: medium
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create .github/workflows/actionlint.yml to validate GitHub Actions workflow syntax.

- Trigger: pull_request on paths: .github/workflows/*
- Job: Download and run actionlint (from rhysd/actionlint via bash download script)
- Check all .github/workflows/*.yml files for syntax errors
- Pin actionlint download via SHA-verified bash invocation

Pattern from SieveEditor's actionlint.yml.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Workflow triggers on PRs touching .github/workflows/
- [ ] #2 Validates all workflow files for syntax errors
- [ ] #3 Action pinned to verified SHA
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
