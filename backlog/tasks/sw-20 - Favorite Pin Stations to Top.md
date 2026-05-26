---
id: SW-20
title: "Favorite/Pin Stations to Top of Library List (#796)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels:
  - feature
milestone: ""
dependencies: []
references:
  - 'https://gitlab.gnome.org/World/Shortwave/-/work_items/796'
  - 'https://gitlab.gnome.org/World/Shortwave/-/issues/617'
  - src/database/models.rs
  - src/database/schema.rs
  - src/database/queries.rs
  - src/database/library.rs
  - src/api/station.rs
  - src/api/station_sorter.rs
  - data/gtk/station_row.ui
  - src/ui/station_row.rs
  - data/gtk/station_dialog.ui
  - src/ui/station_dialog.rs
  - data/database/migrations/
priority: medium
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Je mehr Stationen ein User in seiner Library hat, desto wichtiger wird
es, bestimmte Stationen zu "pinnen" (favorisieren/starren), damit sie
immer oben in der Liste erscheinen — unabhängig von der aktiven Sortierung.

Dieser Task fügt hinzu:
- pinned-Boolean in der Datenbank (neue Migration)
- pinned-Property auf SwStation
- Stern-Button auf StationRow zum Pinnen/Entpinnen
- Pinned-First-Sortierung im SwStationSorter
- Pin-Toggle im StationDialog
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] #1 Neue DB-Migration: pinned BOOLEAN Spalte in library-Tabelle
- [ ] #2 SwStation hat is_pinned/set_pinned GObject-Property mit notify
- [ ] #3 Stern-Toggle-Button auf StationRow (starred/star-outline-symbolic)
- [ ] #4 Gepinnte Stationen erscheinen immer zuerst, unabhängig von Sortierung
- [ ] #5 Pin-Toggle im StationDialog
- [ ] #6 Bestehende Library-Sortierung bleibt erhalten (nur pinned kommt zuerst)
- [ ] #7 Kein Datenverlust bei Migration (Down/Migration rückbaubar)
- [ ] #8 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
### Subtask 1.1 — Datenbank-Migration
- Neues Verzeichnis: data/database/migrations/2026-05-24-000000_add_pinned/
- up.sql: `ALTER TABLE library ADD COLUMN pinned BOOLEAN NOT NULL DEFAULT FALSE;`
  (SQLite unterstützt ALTER TABLE ADD COLUMN — kein temp-table-workaround nötig für eine Spalte)
- down.sql: temp-table-workaround (CREATE tmp ohne Spalte, INSERT, DROP, RENAME)
- src/database/models.rs: `pinned: bool` zu StationEntry
- src/database/schema.rs: pinned-Spalte in diesel schema!() aufnehmen

### Subtask 1.2 — SwStation-Property
- src/api/station.rs: `fn is_pinned()` / `fn set_pinned(bool)` + notify::pinned signal
- src/database/library.rs: beim Laden pinned aus DB setzen, beim Speichern in StationEntry übernehmen
- StationEntry::for_station() muss pinned mitspeichern

### Subtask 1.3 — Stern-Button in StationRow
- data/gtk/station_row.ui: GtkToggleButton mit star-outline-horiz-symbolic / starred-symbolic
  Position: kleine Schaltfläche oben-rechts auf der Card
- src/ui/station_row.rs:
  - Bind toggle → station.pinned bidirectional
  - Stern-Icon wechselt zwischen outlined/filled
  - data/gtk/style.css: Stern bei :hover zeigen, bei pinned immer sichtbar

### Subtask 1.4 — Pinned-First-Sortierung
- src/api/station_sorter.rs: in station_cmp():
  - Zuerst a.is_pinned() vs b.is_pinned() vergleichen
  - pinned > not pinned
  - Wenn beide pinned (oder beide nicht): bestehende Sortierlogik
- Kein neuer Sorting-Enum-Wert nötig (pinned ist immer zuerst)

### Subtask 1.5 — Pin-Toggle in StationDialog
- data/gtk/station_dialog.ui: "An Bibliothek anheften" Toggle-Switch
- src/ui/station_dialog.rs: mit station.bind_property("pinned", ...)
- Direktes Speichern in DB bei toggle (oder beim Dialog-Schließen)
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
- [ ] #2 Manueller Test: Station pinnen → erscheint oben → Sortierung wechseln → bleibt oben
- [ ] #3 Manueller Test: Stern-Button in Row und Dialog funktionieren
- [ ] #4 Manueller Test: DB-Migration up/down ohne Datenverlust
- [ ] #5 cargo clippy --all -- -D warnings pass
<!-- DOD:END -->
