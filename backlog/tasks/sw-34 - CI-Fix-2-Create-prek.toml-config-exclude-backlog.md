---
id: SW-34
title: "CI-Fix-2: Create prek.toml config, exclude backlog/"
status: Done
assignee: []
created_date: 2026-05-28 14:58
updated_date: 2026-05-28 15:08
labels: []
dependencies: []
references:
  - https://github.com/lenucksi/Shortwave/actions/runs/26569090741/job/78271313896
modified_files:
  - prek.toml
  - .gitignore
priority: high
ordinal: 28000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a prek.toml configuration file that migrates the existing .pre-commit-config.yaml hooks and adds proper excludes for backlog/ directory. Also remove backlog/tmp/ from .gitignore since backlog/ is now fully excluded. The existing .pre-commit-config.yaml can remain for backwards compatibility.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 prek run --all-files does not touch files in backlog/
- [ ] #2 backlog/tmp/ removed from .gitignore
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read existing .pre-commit-config.yaml to migrate hooks\n2. Create prek.toml with all hooks from pre-commit-config\n3. Add exclude = [{ glob = ['backlog/**'] }]\n4. Remove backlog/tmp/ line from .gitignore\n5. Test with prek run --all-files (if prek is available)
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created prek.toml with backlog/ exclude glob. Migrated all hooks from .pre-commit-config.yaml. Removed backlog/tmp/ from .gitignore as directed.
<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->