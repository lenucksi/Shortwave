---
id: SW-4
title: 'PR: Playlist Support für upstream (feat/playlist-support)'
status: 'To Do'
assignee: []
created_date: '2026-05-24 16:00'
updated_date: '2026-05-24 16:00'
labels: []
milestone: pr-upstream
dependencies:
  - SW-1
  - SW-2
priority: medium
ordinal: 4000
---

## PR Title

feat: add PLS/M3U playlist support with stream URL fallback

## PR Description

```
## Summary

Shortwave currently only handles single stream URLs. Many radio stations
provide PLS or M3U playlists containing multiple fallback URLs (e.g. Icecast
round-robin). When the primary server fails, the stream stops — even though
alternative URLs exist.

This PR adds playlist parsing, detection in the add-station dialog, storage of
all alternative URLs, and automatic fallback in the player.

## Changes

### Playlist Parsing (new `src/playlist/` module)
- **PLS parser**: Full PLS format support (File1..N, Title1..N,
  NumberOfEntries, comments, CRLF, whitespace-tolerant)
- **M3U/M3U8 parser**: Extended M3U (#EXTM3U, #EXTINF) and bare-URL formats
- **Fetch**: HTTP download with format detection via reqwest
- **17 unit tests** covering happy paths, edge cases, and error cases

### Station Metadata (`src/api/station_metadata.rs`)
- New fields: `alternate_urls: Vec<Url>`, `playlist_url: Option<Url>`,
  `fetched_at: Option<i64>` with `#[serde(default)]` for backward compat

### Player Fallback (`src/audio/player.rs`)
- `try_next_fallback_url()` transparently cycles through URLs on stream failure
- `advance_fallback_url()` extracted pure function with 4 unit tests

### Add-Station Dialog (`src/ui/add_station_dialog.rs`)
- PLS/M3U URL detection triggers async fetch
- Status label shows progress ("Fetching...", "Found N URLs", errors)
- Station name auto-populated from playlist title
- No re-fetch loop on URL field update

### Other
- `src/playlist/mod.rs`: PlaylistEntry struct, PlaylistError enum
- `src/api/station.rs`: stream_urls() convenience method
- Clippy fixes for pre-existing warnings (rustc 1.95)

## Testing
`cargo test` — 25 tests pass (17 parser + 4 serialization + 4 fallback)
`cargo clippy --all -- -D warnings` — clean
```

## ACs

- [ ] #1 PR in GitLab geöffnet
- [ ] #2 Beschreibung passt zum Code
- [ ] #3 Commit-History ist sauber (keine Merge-Commits, keine Tooling-Commits)
