---
id: SW-11
title: "Discovery Sources UI + Ergebnisanzeige"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: ui-integration
dependencies:
  - SW-8
  - SW-9
  - SW-10
references:
  - src/ui/pages/search_page.rs
  - src/ui/pages/library_page.rs
  - data/gtk/search_page.ui
  - data/gtk/discovery_source_row.ui
  - src/ui/station_dialog.rs
  - src/ui/station_row.rs
priority: high
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Die UI für Discovery Sources: Nutzer sehen verfügbare Provider auf der
Search/Discover-Seite, können sie auswählen, ausführen und die gelieferten
Stationen durchsehen und zur Library hinzufügen.

Konkret:
- Neue Sektion "Discovery Sources" unter/neben "Popular Stations"
- Jeder Provider als Klick-Button/Karte mit Name + Beschreibung
- "Run" Action: führt Script aus, zeigt Loading-Spinner
- Ergebnis: Stations werden direkt in die Listenansicht geladen
  (gleiches Design wie Search Results)
- Fehler werden als In-App Notification gezeigt

Die Integration nutzt vorhandene UI-Patterns: SwStationRow, SwStationModel
und die SearchPage-Struktur — minimal neuer UI-Code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Discovery-Sektion auf der SearchPage sichtbar
- [ ] #2 Alle registrierten Provider werden als Liste angezeigt
- [ ] #3 Klick auf Provider → Loading-State + Script Execution
- [ ] #4 Ergebnisse erscheinen als Stations-Liste (SwStationRow)
- [ ] #5 Klick auf Station → SwStationDialog (Add to Library)
- [ ] #6 Fehler-Status: Notification wenn Script fehlschlägt
- [ ] #7 Provider UI aktualisiert sich bei Änderungen im Registry
- [ ] #8 UI-Strings sind i18n-fähig (gettext)
- [ ] #9 cargo clippy pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **data/gtk/discovery_source_row.ui**: Neuer Widget-Template
   - Provider Name (bold)
   - Beschreibung (secondary label)
   - "Load Stations" Button
   - Loading-Spinner (aktiv während Script läuft)

2. **src/ui/discovery_source_row.rs**: GObject Widget
   - `DiscoverySourceRow` mit CompositeTemplate
   - Properties: `provider-id`, `provider-name`, `description`, `loading`
   - Signal: `run-provider` (emitted bei Button-Klick)

3. **SearchPage anpassen** (`src/ui/pages/search_page.rs`):
   - Neue Box/Section "Discovery Sources" unter dem existing Search-Filter
   - Bei App-Start: Provider aus Registry laden → Source Rows
   - Bei "Load Stations": Script ausführen → Ergebnisse in Stations-Liste
   - Gleiches StationRow-Format wie Search Results

4. **UI-Logik**:
   - `run_provider()` → async spawn → Ergebnis in SwStationModel
   - Bei Erfolg: Scroll-to-results, Erfolgs-Notification
   - Bei Fehler: `adw::Toast` mit Fehlertext

5. **i18n**: String-Konstanten via `gettext` / `i18n_f()` Macro
   (existing pattern aus dem Codebase)

6. **Registrierung**: `src/ui/mod.rs` → `mod discovery_source_row;`
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Discovery Sources UI sichtbar + funktional
- [ ] #3 Stations aus Script werden korrekt in der Liste angezeigt
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 User-Test: Provider klicken → Station adden → abspielen
<!-- DOD:END -->
