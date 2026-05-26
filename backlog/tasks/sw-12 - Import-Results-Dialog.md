---
id: SW-12
title: "Import Results Dialog (Preview + Select + Import)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: ui-integration
dependencies:
  - SW-11
references:
  - src/ui/station_dialog.rs
  - data/gtk/station_dialog.ui
  - data/gtk/discovery_results_dialog.ui
  - src/ui/add_station_dialog.rs
  - src/database/library.rs
priority: medium
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ein Dialog, der die vom Discovery-Script gelieferten Stationen
anzeigt, bevor sie in die Library übernommen werden. Der Nutzer
kann einzelne Stationen auswählen/abwählen, Details sehen und
dann gezielt importieren.

Funktionen:
- Liste aller gefundenen Stationen (Name + Tags + Codec/Bitrate)
- Checkbox pro Station (alle vor-selektiert)
- Station-Detail-Preview (homepage, icon, url) via Klick
- "Import Selected (N)" Button
- "Cancel" schließt ohne Änderungen
- Suche/Filter innerhalb der Ergebnisse

Inspiration: Der existing SwStationDialog + SwAddStationDialog,
angepasst für Multi-Station-Import.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Dialog öffnet sich nach erfolgreichem Discovery-Run
- [ ] #2 Alle Stationen werden als Liste mit Checkboxen angezeigt
- [ ] #3 Alle Checkboxen sind default an
- [ ] #4 Auswahlzähler: "Import Selected (12/30)"
- [ ] #5 Station-Detail-Preview bei Klick (Name, URL, Tags, Icon)
- [ ] #6 "Import Selected" fügt alle selected in Library (db + model)
- [ ] #7 "Cancel" schließt ohne Änderungen
- [ ] #8 Suche/Filter innerhalb der Ergebnisse
- [ ] #9 Erfolgs-Notification: "12 stations imported from SomaFM"
- [ ] #10 Dialog ist resizable
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **data/gtk/discovery_results_dialog.ui**:
   - `AdwWindow` oder `AdwDialog` (AdwDialog ist ab libadwaita 1.7+)
   - Header mit Provider-Name + "X stations found"
   - Search-Entry für Filter
   - ScrolledWindow mit Station-Liste (je Row: Checkbox + Name + Tags)
   - Detail-Preview (rechts oder als Popover)
   - Bottom-Bar: "Import Selected (N)" Button + Cancel

2. **src/ui/discovery_results_dialog.rs**:
   - `DiscoveryResultsDialog` als GObject Widget
   - Properties: `provider-name`, `stations` (Vec<StationData>)
   - `fn run(parent, provider, stations) -> impl Future<Output = Vec<StationData>>`
   - Signal: `stations-imported` mit den importierten Stationen

3. **Import-Logik**:
   - StationData → StationMetadata konvertieren (UUID generieren, is_local = true)
   - `SwLibrary::add_station()` für jede Station
   - Bulk-Insert für Performance (innerhalb einer Transaktion)

4. **Integration mit SW-11**:
   - DiscoverySourceRow öffnet nach erfolgreichem Run den Dialog
   - Dialog gibt importierte Stationen zurück → Library + Model Update
   - Notification nach Import

5. **i18n**: Alles via gettext
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Dialog öffnet + Stations werden angezeigt
- [ ] #3 Import in Library funktioniert
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 User-Test: Dialog + Import + Abspielen
<!-- DOD:END -->
