---
id: SW-15
title: "Custom Provider Management UI"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: tooling
dependencies:
  - SW-11
references:
  - src/discovery/registry.rs
  - src/settings/settings_manager.rs
  - data/gtk/add_provider_dialog.ui
  - src/ui/discovery_source_row.rs
priority: low
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Nutzer können eigene Extraction Scripts in die App einbinden —
ohne Kommandozeile. Ein Dialog erlaubt das Hinzufügen, Entfernen
und Verwalten von custom `.rhai` Providern.

Features:
- "Add Custom Provider" Button in der Discovery-Sektion
- File-Chooser für `.rhai` Dateiauswahl
- Nach Hinzufügen: Provider erscheint sofort in der Liste
- "Remove Provider" via Rechtsklick/Kontextmenü
- Provider Enable/Disable Toggle
- Provider-Settings persistent (GSettings oder JSON in config dir)
- "Open Provider Folder" Button (öffnet Dateimanager)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 "Add Custom Provider" Button in Discovery-Sektion
- [ ] #2 File-Chooser filtert auf `.rhai` Dateien
- [ ] #3 Provider wird in `~/.local/share/shortwave/providers/` kopiert
- [ ] #4 Provider erscheint sofort (Registry-Rescan)
- [ ] #5 Remove-Button löscht Datei + entfernt aus Registry
- [ ] #6 Enable/Disable-Preference wird persistiert
- [ ] #7 "Open Provider Folder" öffnet File-Manager
- [ ] #8 Drag & Drop von .rhai Dateien (nice-to-have)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **Provider-Persistenz**:
   - Enable/Disable State → GSettings oder JSON-File pro Provider
   - Vorschlag: GSettings mit `provider-enabled` Array-Key
   - Oder: `.provider-state.json` im user-provider-dir

2. **Add-Provider-Dialog**:
   - `AdwDialog` oder `GtkFileChooserNative`
   - File-Filter: `*.rhai`
   - Nach Auswahl: Datei in `~/.local/share/shortwave/providers/` kopieren
   - Registry-Scan triggern → UI update

3. **Kontextmenü**:
   - Rechtsklick auf DiscoverySourceRow → "Remove", "Open File"
   - Entfernen: `.rhai` Datei löschen (oder in Trash verschieben)
   - Registry + UI update

4. **Enable/Disable**:
   - Switch in der Source-Row
   - State wird persistiert
   - Disabled Provider werden in der UI ausgegraut

5. **Open Provider Folder**:
   - `gtk::show_uri(parent, "file:///home/user/.local/share/...")`
   - Nutzt `gdk::AppLaunchContext` oder `glib::open_uri()`
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 User kann custom Provider hinzufügen + nutzen
- [ ] #3 `cargo clippy --all -- -D warnings` pass
- [ ] #4 User-Test: Custom .rhai importieren → ausführen → Station adden
<!-- DOD:END -->
