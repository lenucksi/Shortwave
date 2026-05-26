---
id: SW-5
title: 'PR: Multi-URL Anzeige + LogoURL für upstream (feat/playlist-detail)'
status: 'To Do'
assignee: []
created_date: '2026-05-24 16:00'
updated_date: '2026-05-24 16:00'
labels: []
milestone: pr-upstream
dependencies:
  - SW-3
  - SW-4
priority: medium
ordinal: 5000
---

## PR Title

feat: show all stream URLs in station dialog + PLS LogoURL support

## PR Description

```
## Summary

When a station was added via a PLS/M3U playlist, it may have multiple
stream URLs stored in `alternate_urls`. The station detail dialog
previously showed only the primary URL. This PR displays all URLs
with a live indicator for the currently playing stream.

Additionally, PLS supports a LogoURL field. This is now parsed and
stored as the station favicon.

## Changes

### Multi-URL Display (`src/ui/station_dialog.rs`)
- Station dialog dynamically builds AdwActionRow entries — one per stream URL
- Currently active URL is highlighted with bold text and ▶ prefix
- Copy button per URL with clipboard + toast notification
- Active indicator updates live when playback state changes (connect_notify)
- Each URL row is stored with URL tracking for live updates

### PLS LogoURL (`src/playlist/pls.rs`, `src/playlist/mod.rs`)
- PlaylistEntry: new `logo_url: Option<Url>` field
- PLS parser: `LogoURL=<url>` extracted as station-wide logo
- AddStationDialog: sets `metadata.favicon` from `first.logo_url`

### Player (`src/audio/player.rs`)
- `SwPlayer::active_stream_url()` public method returns currently
  playing URL based on `current_url_index` and `fallback_urls`

### Bugfixes (backported to feat/playlist-support)
- `update_metadata()` no longer overwrites `alternate_urls` with
  `..Default::default()` after playlist fetch (dedup check moved before set_metadata)
- Add dialog content_height adjusted to prevent scrolling (550 → 585)

## Testing
`cargo test` — 25 tests pass
`cargo clippy --all -- -D warnings` — clean
```

## ACs

- [ ] #1 PR in GitLab geöffnet
- [ ] #2 Beschreibung passt zum Code
- [ ] #3 Commit-History ist sauber (keine Merge-Commits, keine Tooling-Commits)
