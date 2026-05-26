---
id: SW-3
title: Multi-URL Anzeige im Station-Dialog + PLS LogoURL
status: 'Done'
assignee: []
created_date: '2026-05-24 12:30'
updated_date: '2026-05-24 16:00'
labels: []
milestone: playlist-detail
dependencies:
  - SW-1
references:
  - src/playlist/mod.rs
  - src/playlist/pls.rs
  - src/ui/add_station_dialog.rs
  - src/audio/player.rs
  - src/ui/station_dialog.rs
  - data/gtk/station_dialog.ui
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Stationen die per PLS/M3U geadded wurden haben mehrere Stream-URLs (alternate_urls). Der Station-Dialog zeigt aktuell nur die primäre URL. Er soll alle URLs listen mit Markierung der aktiven.

Zusätzlich: PLS unterstützt LogoURL, die als favicon in die StationMetadata übernommen werden soll.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [x] #1 Station-Dialog zeigt alle URLs aus alternate_urls
- [x] #2 Aktive URL ist hervorgehoben (▶ Prefix + bold markup)
- [x] #3 Copy-Button pro URL
- [x] #4 PLS LogoURL wird als favicon auf StationMetadata gesetzt
- [x] #5 Player active_stream_url() korrekt
- [x] #6 cargo fmt + cargo clippy --all -- -D warnings pass
- [x] #7 Human signoff: UI funktioniert (2026-05-24)
<!-- AC:END -->

## Definition of Done

<!-- DOD:BEGIN -->
- [x] #1 Tests wo möglich (Parser, PlaylistEntry serialization)
- [x] #2 Branch auf feat/playlist-support basiert, nicht upstream main
- [ ] #3 Nach Merge: Branch rebasen für cleanen PR (tooling commits droppen)
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
### PLS LogoURL (src/playlist/pls.rs)
- Neue LogoURL-Parsing: `if key.eq_ignore_ascii_case("LogoURL")` extrahiert station-weites Logo
- Wird auf PlaylistEntry.logo_url des ersten Eintrags gesetzt
- AddStationDialog setzt metadata.favicon aus first.logo_url

### PlaylistEntry (src/playlist/mod.rs + m3u.rs)
- Neues Feld: `pub logo_url: Option<Url>` — Default None

### Player (src/audio/player.rs)
- Neue Methode: `SwPlayer::active_stream_url()` — liest `current_url_index` und `fallback_urls` aus

### Station-Dialog (data/gtk/station_dialog.ui + src/ui/station_dialog.rs)
- Template: statische `stream_row` ersetzt durch `audio_group` (AdwPreferencesGroup mit id)
- setup_widgets: baut pro URL eine AdwActionRow mit Copy-Button + url_rows RefCell
- Aktive URL: <b>▶ url</b> — Bold + Prefix, live via update_url_indicators()
- Signal: connect_notify_local("state"/"station") aktualisiert Indikator bei Playback-Änderung
- Copy: Clipboard + Toast via ToastOverlay
- Alte copy_stream_clipboard-Callback entfernt

### update_metadata Bugfix (add_station_dialog.rs)
- dedup check (pls_fetched_url) VOR set_metadata() — sonst wird alternate_urls mit ..Default::default() überschrieben
- async callback: pls_fetched_url auf stream URL setzen vor set_text
- content_height 585 gegen Scrollen (fine-tuned: 550→585)

### live URL indicator (station_dialog.rs)
- url_rows: RefCell<Vec<(Url, ActionRow)>> speichert generierte Zeilen
- update_url_indicators() aktualisiert ▶ bei player.state() == Playing
- Keine Leaks: glib::clone! mit #[weak] Referenzen<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINALSUMMARY:BEGIN -->
SW-3 komplett. Alle 7 ACs erfüllt + User-Signoff. Branch `feat/playlist-detail` basiert auf `feat/playlist-support` (stacked PR). 25 Tests pass, clippy clean. Backport von update_metadata Bugfix nach `feat/playlist-support`. `content_height` 585 fine-tuned gegen Scrollen.
<!-- SECTION:FINALSUMMARY:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. PlaylistEntry um logo_url: Option<Url> erweitern
2. PLS Parser: LogoURL=<url> parsen
3. AddStationDialog: logo_url -> metadata.favicon setzen
4. SwPlayer: fn active_stream_url(&self) -> Option<Url> + current_url_index Property
5. station_dialog.ui: URL-Liste unter Audio (eine Zeile pro URL) + Copy-Buttons
6. station_dialog.rs: Multi-URL Setup + Copy-Buttons + Active-Indikator
7. Tests + Clippy + Rebuild
<!-- SECTION:PLAN:END -->
