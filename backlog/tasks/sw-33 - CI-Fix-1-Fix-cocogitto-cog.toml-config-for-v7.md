---
id: SW-33
title: "CI-Fix-1: Fix cocogitto cog.toml config for v7"
status: Done
assignee: []
created_date: 2026-05-28 14:58
updated_date: 2026-05-28 15:08
labels: []
dependencies: []
references:
  - https://github.com/lenucksi/Shortwave/actions/runs/26569090741/job/78271313914
modified_files:
  - cog.toml
priority: high
ordinal: 27000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Fix tag_prefix position in cog.toml. In cocogitto 7.x, tag_prefix must be a top-level setting before [changelog], not after it (where it falls into the changelog table scope). Also remove the duplicate at the bottom of the file. Validate locally with 'cog check --from-latest-tag'.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cog check --from-latest-tag passes locally
- [ ] #2 No duplicate tag_prefix in cog.toml
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read current cog.toml\n2. Move tag_prefix before [changelog]\n3. Remove duplicate tag_prefix after [commit_types]\n4. Run cog check --from-latest-tag
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Moved tag_prefix to top-level (before [changelog]), removed duplicate. Config parses correctly with cog 7.0.0. The commit check errors are historical non-conventional commits from renovate/dependabot bots.
<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
- [ ] #5 Config validated
<!-- DOD:END -->