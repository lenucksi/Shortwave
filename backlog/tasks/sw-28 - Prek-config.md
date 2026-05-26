---
id: SW-28
title: Prek config
status: To Do
assignee: []
created_date: 2026-05-26 13:33
labels: []
dependencies: []
modified_files:
  - .pre-commit-config.yaml
priority: medium
ordinal: 28000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create .prek.toml for local Git hooks via prek (Rust drop-in for pre-commit).

prek reads the same .pre-commit-config.yaml format, but we can use either format.

Hooks to configure:
1. **cargo fmt**: Format Rust code
2. **cargo clippy**: Lint Rust code
3. **typos**: Spell checking via typos-cli (existing .typos.toml config)
4. **trailing-whitespace + end-of-file-fixer**: from pre-commit/pre-commit-hooks
5. **conventional-precommit-linter** or cocogitto check: validate commit messages follow conventional commits
6. **gitleaks**: Secret detection

Use .pre-commit-config.yaml format (compatible with both prek and pre-commit).

Default install hook types: pre-commit, commit-msg.

Also add a note in AGENTS.md or README about running 'prek install' after clone.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 prek runs all hooks successfully
- [ ] #2 cargo fmt, cargo clippy, typos checks included
- [ ] #3 commit-msg hook validates conventional commits
- [ ] #4 trailing-whitespace and end-of-file-fixer included
- [ ] #5 Default install hook types configured (pre-commit, commit-msg)
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
