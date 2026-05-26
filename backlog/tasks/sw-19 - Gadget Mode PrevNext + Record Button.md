---
id: SW-19
title: "Gadget Mode: Prev/Next Station + Record Buttons (#801 adjunct)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels:
  - feature
milestone: ""
dependencies:
  - SW-18
references:
  - 'https://gitlab.gnome.org/World/Shortwave/-/work_items/801'
  - data/gtk/player_gadget.ui
  - src/ui/player/player_gadget.rs
priority: medium
ordinal: 19000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Gadget Mode (kompaktes Mini-Player-Fenster, per max-height:165px
oder Button aktiviert) hat aktuell nur Play/Stop, Lautstärke und
Station-Name. Es fehlen:
- Prev/Next Station-Buttons zum Durchschalten der Library
- Ein Record-Button für Session-Recording (SW-18)

Dieser Task ergänzt diese Buttons im Gadget Mode.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] #1 Prev/Next Buttons im Gadget (media-skip-backward/forward-symbolic)
- [ ] #2 Prev/Next cyclen durch die sortierte Library-Liste
- [ ] #3 Record Button im Gadget (media-record-symbolic, toggle)
- [ ] #4 Record-Button zeigt Recording-State (rot/pulsierend wenn aktiv)
- [ ] #5 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
### Subtask 1.1 — Prev/Next Buttons
- data/gtk/player_gadget.ui: Zwei Buttons (media-skip-backward/forward) neben Play/Stop
- src/ui/player/player_gadget.rs:
  - Zugriff auf das SwStationModel (sortiert via SwStationSorter) aus der Library
  - Bei "next": next station in der sortierten Liste auswählen (→ player.set_station + player.start_playback)
  - Bei "prev": previous station auswählen
  - Wrap-Around oder Stop am Ende? → am Ende bleiben (kein Wrap-Around)

### Subtask 1.2 — Record Button
- data/gtk/player_gadget.ui: GtkToggleButton mit media-record-symbolic
- src/ui/player/player_gadget.rs:
  - Verbindung zu SwPlayer Recording (aus SW-18)
  - An: rot/pulsierend, Dauer-Anzeige (klein)
  - Aus: grau, untoggled
  - Während Recording: Button leuchtet rot → Klick stoppt Recording
  - Ohne Recording: Klick startet Recording
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->

<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->

<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done

<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Manueller Test: Gadget-Mode öffnen → Prev/Next schaltet Stationen
- [ ] #3 Manueller Test: Record im Gadget starten/stoppen
- [ ] #4 cargo clippy --all -- -D warnings pass
<!-- DOD:END -->
