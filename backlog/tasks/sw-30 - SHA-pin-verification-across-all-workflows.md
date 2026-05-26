---
id: SW-30
title: SHA pin verification across all workflows
status: To Do
assignee: []
created_date: 2026-05-26 13:33
labels: []
dependencies: []
priority: high
ordinal: 30000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Cross-cutting task: before committing any GitHub Actions workflow files, verify every action reference is pinned to a verified full commit SHA.

For each action in .github/workflows/*.yml:
1. Identify the action owner/repo (e.g. actions/checkout, flatpak/flatpak-github-actions)
2. Run: git ls-remote https://github.com/{owner}/{repo}.git to list tags
3. Find the latest stable version tag
4. Verify the SHA for that exact tag
5. Use the full commit SHA in the action reference
6. Append a comment: # vX.Y.Z

Actions to verify (not exhaustive, final list from all workflow files):
- actions/checkout
- actions/upload-artifact
- actions/download-artifact
- actions/attest-build-provenance
- actions/upload-pages-artifact
- actions/deploy-pages
- flatpak/flatpak-github-actions/flatpak-builder
- crazy-max/ghaction-import-gpg
- cocogitto/cocogitto-action
- j178/prek-action
- dorny/paths-filter
- step-security/harden-runner
- actions/create-github-app-token
- peter-evans/create-pull-request (if used)
- rhysd/actionlint (download script)

IMPORTANT: Each action is a separate repo with different SHAs. Never reuse SHAs between different actions, even at the same version number (e.g. upload-artifact v7 and download-artifact v8 have different SHAs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every action in every workflow file uses a full commit SHA, not a tag
- [ ] #2 Each SHA verified against its specific upstream repo via git ls-remote
- [ ] #3 Actions from different repos never share SHAs
- [ ] #4 SHA pinned version documented in inline comment (# vX.Y.Z)
- [ ] #5 Applies to all workflow files: ci.yml, flatpak.yml, release.yml, actionlint.yml
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->
