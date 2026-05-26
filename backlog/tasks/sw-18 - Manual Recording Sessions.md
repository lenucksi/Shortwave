---
id: SW-18
title: "Manual Recording Sessions (Track-Separated, No Duration Limit) (#801)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels:
  - feature
milestone: ""
dependencies: []
references:
  - 'https://gitlab.gnome.org/World/Shortwave/-/work_items/801'
  - src/audio/recording_mode.rs
  - src/audio/player.rs
  - src/audio/gstreamer_backend.rs
  - src/audio/track.rs
  - src/ui/recording_indicator.rs
  - data/gtk/recording_indicator.ui
  - src/ui/preferences_dialog.rs
  - data/de.haeckerfelix.Shortwave.gschema.xml.in
  - src/settings/key.rs
priority: high
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Das aktuelle Recording ist automatisch: Es startet bei Play, splittet
Tracks bei ICY-Titelwechsel und hat ein hartes 15-Min-Limit.

User wollen: "Start Recording" klicken → bekomme einen Ordner mit
ordentlich getrennten Songs mit Titeln → "Stop Recording" klicken.

Dieser Task baut ein manuelles Session-Recording:
- Expliziter Start/Stop-Button (kein Auto-Start)
- Tracks werden weiterhin bei ICY-Titelwechsel getrennt
- Jeder Track als eigene Datei: {Artist} - {Title}.ogg
- Alle Tracks in einem datierten Session-Ordner
- Kein Zeitlimit (nur durch Plattenplatz begrenzt)
- Beim Stop: aktuellen Track finalisieren, Ordner öffnen
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] #1 "Start Recording" / "Stop Recording" Button in Player-View und Toolbar
- [ ] #2 Klick auf Start beginnt Aufnahme, Stop beendet sie (kein Auto-Start bei Play)
- [ ] #3 Tracks werden bei ICY-Titelwechsel getrennt (wie bisher)
- [ ] #4 Jeder Track als separate .ogg-Datei im Session-Ordner
- [ ] #5 Session-Ordner: {RecordingTrackDirectory}/Shortwave/{Station}/{YYYY-MM-DD}/
- [ ] #6 Kein max-duration-Limit während Session-Recording
- [ ] #7 Stop finalisiert aktuellen Track + öffnet Ordner im File-Manager
- [ ] #8 Alte Recording-Modi (Decide/Everything/Nothing) bleiben optional erhalten
- [ ] #9 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
### Subtask 1.1 — Neues Recording-Session-Modell
In src/audio/recording_mode.rs:
- `ManualSession` oder `Session` Variante zu RecordingMode hinzufügen
- In src/audio/player.rs:
  - recording_start() / recording_stop() als explizite Aktionen
  - Kein Auto-Start in gst_title_change() wenn Session-Mode (nur split)
  - gst_title_change() erzeugt neuen Track im selben Session-Ordner
  - recording_maximum_duration-Ignorieren im Session-Mode
  - Dateiname: sanitize_filename!("{Artist} - {Title}.ogg")

### Subtask 1.2 — Session-Ordner-Logik
- Session-Ordner: {RecordingTrackDirectory}/Shortwave/{StationName}/{YYYY-MM-DD}/
- Ordnernamen sanitizen
- Beim ersten Record einer Session: Ordner erstellen
- Beim Stop: `xdg-open` oder `gio open` auf den Ordner

### Subtask 1.3 — UI für Recording-Controls
- src/ui/recording_indicator.rs + data/gtk/recording_indicator.ui:
  - "Start Recording" (roter Kreis) / "Stop Recording" (rotes Quadrat) Button
  - Während Recording: pulsierend, Dauer + Track-Count anzeigen
  - In Player-View, Toolbar und Gadget verfügbar

### Subtask 1.4 — Preferences
- src/ui/preferences_dialog.rs + data/gtk/preferences_dialog.ui:
  - Bestehende Recording-Modi (Decide/Everything/Nothing) als Option erhalten
  - "Session Recording" als zusätzliche Option
  - recording-track-directory bleibt als Basis-Pfad
  - GSchema: recording-session-mode bool key (optional)

### Subtask 1.5 — Migration alter Modi (optional)
- Wenn Decide/Everything/Nothing beibehalten: weiterhin max-duration
- Session-Mode: kein duration-limit
- User kann zwischen Modi in Preferences wählen
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
- [ ] #2 Manueller Test: Start drücken → Aufnahme läuft → Trackwechsel → Stop → Ordner mit Dateien
- [ ] #3 Manueller Test: Aufnahme >15min ohne Abbruch
- [ ] #4 Manueller Test: Alte Recording-Modi funktionieren noch
- [ ] #5 cargo clippy --all -- -D warnings pass
<!-- DOD:END -->
