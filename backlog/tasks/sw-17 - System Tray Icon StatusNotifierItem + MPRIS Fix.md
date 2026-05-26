---
id: SW-17
title: "System Tray Icon (StatusNotifierItem) + MPRIS Fix (#813)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels:
  - bug
  - feature
milestone: ""
dependencies: []
references:
  - 'https://gitlab.gnome.org/World/Shortwave/-/work_items/813'
  - 'https://github.com/Sanjai-Shaarugesh/Advanced-media-controller/issues/24'
  - src/audio/mpris.rs
  - src/audio/player.rs
  - src/app.rs
  - src/ui/window.rs
  - Cargo.toml
  - src/tray/mod.rs
  - build-aux/de.haeckerfelix.Shortwave.Devel.json
priority: high
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Wenn Shortwave im Hintergrund läuft (background-playback, standardmäßig aktiv),
gibt es keine Möglichkeit zu sehen, was gerade läuft, ohne das Fenster
wiederherzustellen. MPRIS funktioniert beim User nicht zuverlässig — Pause
lässt Shortwave aus dem Media-Panel verschwinden.

Dieser Task fügt hinzu:
- Fix für MPRIS-Deregistrierung bei Pause
- StatusNotifierItem (SNI) für System Tray nach dem
  org.kde.StatusNotifierItem/freedesktop-Standard
- Context Menu im Tray: Play/Stop, Fenster anzeigen, Beenden
- Integration mit Window-close + background-playback
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] #1 MPRIS: Pause deregistriert Shortwave nicht mehr vom D-Bus — App bleibt im Media-Panel sichtbar
- [ ] #2 StatusNotifierItem zeigt aktuellen Station-Namen + Playback-Status im Tray
- [ ] #3 SNI-Context-Menü: Play/Stop (Toggle), Fenster anzeigen, Beenden
- [ ] #4 SNI erscheint beim Schließen des Fensters wenn background-playback=true
- [ ] #5 Wenn background-playback=false → App quitet normal ohne SNI
- [ ] #6 Flatpak: --talk-name=org.kde.StatusNotifierWatcher mit Graceful-Fallback
- [ ] #7 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
### Subtask 1.1 — MPRIS-Deregistrierungs-Bug fixen
In src/audio/mpris.rs + src/audio/player.rs:
- Übergang Play → Pause darf nicht das MPRIS-Server-Objekt zerstören
- playback_status muss korrekt zwischen Playing ↔ Paused wechseln
- D-Bus-Name registration bleibt bestehen
- Test: Nach Pause ist Shortwave noch im MPRIS-Panel sichtbar

### Subtask 1.2 — StatusNotifierItem implementieren
Neues Modul src/tray/mod.rs:
- D-Bus-Protokoll org.kde.StatusNotifierItem implementieren
- Entweder mit `status-notifier`-Crate oder raw `zbus` (bereits im Tree)
- Kategorie: ApplicationStatus, Status: Active/Passive
- Tooltip: "Shortwave — {Station}" oder "Shortwave (nicht am spielen)"
- Icon: App-Icon aus data/icons/
- Context Menu: Play/Stop (toggle), Fenster anzeigen, Beenden
- Verbindung zu player.state + player.station für Live-Updates

### Subtask 1.3 — App-Lifecycle-Integration
In src/app.rs + src/ui/window.rs:
- Window.close: wenn background-playback=true → SNI zeigen (nicht quitten)
- Wenn background-playback=false → normal quitten
- SNI activate (Doppelklick) → Fenster wiederherstellen/anheben
- SNI quit → cleanup + app.quit()
- Wenn keine StatusNotifierWatcher verfügbar → graziler Fallback (kein SNI, kein Fehler)

### Subtask 1.4 — Flatpak
- build-aux/*.json: --talk-name=org.kde.StatusNotifierWatcher
- Oder Portal API org.freedesktop.portal.StatusNotifier testen
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
- [ ] #2 Manueller Test: Pause in MPRIS-fähiger DE → App bleibt sichtbar
- [ ] #3 Manueller Test: Fenster schließen mit background-playback → Tray-Icon erscheint
- [ ] #4 Manueller Test: SNI-Context-Menü-Aktionen funktionieren
- [ ] #5 Manueller Test: background-playback=false → App quitet normal
- [ ] #6 cargo clippy --all -- -D warnings pass
<!-- DOD:END -->
