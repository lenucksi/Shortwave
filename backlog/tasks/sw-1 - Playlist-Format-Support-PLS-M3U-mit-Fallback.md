---
id: SW-1
title: Playlist-Format Support (PLS + M3U) mit Fallback
status: Done
assignee: []
created_date: '2026-05-24 09:19'
updated_date: '2026-05-24 10:33'
labels: []
milestone: playlist-support
dependencies: []
references:
  - 'https://en.wikipedia.org/wiki/PLS_(file_format)'
  - 'https://en.wikipedia.org/wiki/M3U'
  - src/api/station_metadata.rs
  - src/api/station.rs
  - src/audio/player.rs
  - src/audio/gstreamer_backend.rs
  - src/ui/add_station_dialog.rs
  - data/gtk/add_station_dialog.ui
  - src/api/http.rs
  - Cargo.toml
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Shortwave kann aktuell nur einzelne Stream-URLs verarbeiten.
Viele Sender liefern PLS- oder M3U-Playlists, die mehrere Fallback-URLs
enthalten (Icecast round-robin). Wenn der primäre Server ausfällt, bricht
der Stream ab — obwohl alternative URLs existieren.

Dieser Task fügt hinzu:
- PLS und M3U Playlist-Parsing
- Playlist-Erkennung im Add-Station-Dialog
- Multi-URL Speicherung in StationMetadata
- Automatischen URL-Fallback im Player bei Stream-Failure
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PLS-URL wird erkannt, gefetched und geparst
- [x] #2 M3U-URL wird erkannt, gefetched und geparst
- [x] #3 Alle Playlist-URLs in StationMetadata.alternate_urls persistiert
- [x] #4 Add-Dialog zeigt Status während Fetch
- [x] #5 Bei Stream-Failure: transparenter Fallback zur nächsten URL
- [x] #6 Fallback-Erschöpfung: normaler Failure-State + Notification
- [x] #7 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Playlist-Parser Modul (src/playlist/): pls.rs, m3u.rs, fetch.rs, mod.rs mit PlaylistEntry-Struct
2. StationMetadata erweitern: alternate_urls: Vec<Url> + playlist_url + fetched_at mit #[serde(default)]
3. SwStation API: fn stream_urls(&self) -> Vec<Url>
4. Add-Station-Dialog: PLS/M3U detection, async fetch+parse, Status-Label, auto-fill name
5. Player Fallback: fallback_urls + current_url_index in SwPlayer, try_next_fallback_url()
6. Unit-Tests: PLS-Parser, M3U-Parser, Serialisierung, Fallback-Strategie
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented:
- src/playlist/pls.rs: PLS parser with 9 unit tests
- src/playlist/m3u.rs: M3U/M3U8 parser with 8 unit tests
- src/playlist/fetch.rs: HTTP fetch + format detection via reqwest
- src/playlist/mod.rs: PlaylistEntry, PlaylistError exports
- src/api/station_metadata.rs: alternate_urls, playlist_url, fetched_at fields with #[serde(default)]
- src/api/station.rs: fn stream_urls() returning primary+alternate URLs
- src/audio/player.rs: fallback_urls + current_url_index + try_next_fallback_url()
- src/ui/add_station_dialog.rs: PLS/M3U detection, async fetch, status label
- data/gtk/add_station_dialog.ui: pls_status GtkLabel
- src/main.rs: mod playlist;
- src/audio/gstreamer_backend.rs: collapsible match clippy fix

Clippy warnings from both new and pre-existing code were fixed (10 errors from rustc 1.95 clippy update).

No new crates added. All parsing done manually (PLS/M3U are simple text formats).

## UX-Fixes nach User-Test (2026-05-24)
- pls_status Label aus GtkScrolledWindow in AdwToolbarView type="bottom" verschoben — kein Scrollen mehr
- pls_fetched_url Flag verhindert Re-Fetch-Loop bei name_row.set_text()
- Format-Strings korrigiert: {} statt {name} für i18n_f() (nutzt freplace mit {} Platzhaltern)
- Add-Button wird nach erfolgreichem Fetch reaktiviert
- Name wird aus PLS-Title auto-populated, URL wird durch erste Stream-URL ersetzt
- Binary-DATADIR auf tmp/shortwave-dev/show umgestellt (kein /tmp mehr)
- User-Signoff: Dialog funktioniert, Station wird abgespielt
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
SW-1 komplett implementiert und getestet. Alle 7 ACs erfüllt. 17 Parser-Tests + 4 Serialisierungs-Tests + 4 Fallback-Tests = 25 Tests total. Fehlt nur noch Merge in lokales main.

Ursprüngliche 7 ACs + UX-Fixes aus User-Test abgeschlossen. Dialog zeigt Inhalt, PLS/M3U-Fetch funktioniert, Name wird auto-populated, Add-Button reaktiviert, kein Re-Fetch-Loop, kein Scrolling. Station wird abgespielt. Commit: df00a15
<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Alle AC-Items checked
- [x] #2 Parser Unit-Tests in src/playlist/pls.rs und src/playlist/m3u.rs
- [x] #3 Branch gemerged in lokales main
- [x] #4 Keine neuen Crates in Cargo.toml
<!-- DOD:END -->
