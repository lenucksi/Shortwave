---
id: SW-21
title: "Network Connectivity Monitoring via Portal + Meaningful Offline Messages (#783)"
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
  - 'https://gitlab.gnome.org/World/Shortwave/-/work_items/783'
  - 'https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html'
  - src/connectivity/mod.rs
  - src/audio/player.rs
  - src/audio/gstreamer_backend.rs
  - src/database/library_status.rs
  - src/database/library.rs
  - src/ui/pages/library_page.rs
  - data/gtk/library_page.ui
  - data/gtk/style.css
  - Cargo.toml
  - build-aux/
priority: high
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Wenn das Netzwerk ausfällt, erlaubt Shortwave weiterhin Play zu drücken
und scheitert dann mit "Stream doesn't contain enough data" — in einem
unwiederherstellbaren Error-State (Play-Button bleibt kaputt).

Dieser Task fügt hinzu:
- NetworkMonitor via xdg-desktop-portal (ashpd) für Flatpak-Sicherheit
- is_online-Property auf SwPlayer
- Player startet nicht ohne Netz (saubere Fehlermeldung)
- Bei Netzverlust während Play: Warning-Toast
- Fix für unrecoverable Failure-State
- Offline-Banner in der Library-UI
- Play-Buttons deaktivieren wenn offline
- SwLibraryStatus::Offline endlich verwenden (existiert bereits)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] #1 ashpd::desktop::network_monitor::NetworkMonitor überwacht Konnektivität
- [ ] #2 SwPlayer hat is_online-Property, die bei Netzänderungen feuert
- [ ] #3 start_playback() checkt is_online → saubere Fehlermeldung wenn offline
- [ ] #4 "Stream doesn't contain enough data" wird in "Keine Netzwerkverbindung" übersetzt
- [ ] #5 Nach Failure: stop_playback() funktioniert → Play-Button wird wieder aktiv
- [ ] #6 Offline-Banner in Library: "Du bist offline. Stelle eine Verbindung her."
- [ ] #7 Play-Buttons in StationRows sind deaktiviert wenn offline
- [ ] #8 SwLibraryStatus::Offline wird gesetzt (existiert bereits, ungenutzt)
- [ ] #9 Funktioniert in Flatpak + nativ (Fallback auf gio::NetworkMonitor)
- [ ] #10 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
### Subtask 1.1 — ConnectivityMonitor via Portal
Neues Modul src/connectivity/mod.rs:
- Wrap ashpd::desktop::network_monitor::NetworkMonitor (async)
- Expose is_online: bool property + signal "connectivity-changed"
- Bei Init: aktuellen State abfragen
- Fallback wenn kein Portal (nativ): gio::NetworkMonitor::default()
  + connect notify::network-available
- Cargo.toml: ggf. ashpd Features erweitern (network-monitor)

### Subtask 1.2 — Player-Integration
src/audio/player.rs:
- is_online-Property + Verknüpfung mit ConnectivityMonitor
- start_playback(): vor GStreamer-Start is_online checken
  - wenn false: last_failure = "Keine Internetverbindung", state = Stopped, return
- connect_network_monitor(): Callback für connectivity-changed
  - offline während playback → AdwToast "Netzwerkverbindung verloren — Stream kann abbrechen"
  - online → ggf. gescheiterte Station erneut versuchen (optional)

### Subtask 1.3 — Unrecoverable Failure fixen
src/audio/player.rs + src/audio/gstreamer_backend.rs:
- Failure-Error-String "Stream doesn't contain enough data" abfangen
  → übersetzen in "Keine Netzwerkverbindung" (oder "Stream nicht erreichbar")
- stop_playback() muss immer gehen, auch im Failure-State
- Sicherstellen dass Play-Button nach Failure klickbar ist

### Subtask 1.4 — UI Offline-State
src/ui/pages/library_page.rs + data/gtk/library_page.ui:
- AdwBanner "Keine Internetverbindung" wenn Offline
- src/database/library_status.rs: SwLibraryStatus::Offline verwenden
- src/database/library.rs: update_library_status() setzt Offline wenn !is_online
- src/ui/station_row.rs: play_button.set_sensitive(!offline)
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
- [ ] #2 Manueller Test: Flugmodus → Play → "Keine Internetverbindung" Meldung
- [ ] #3 Manueller Test: Während Play Netz aus → Toast → wieder an → Play geht wieder
- [ ] #4 Manueller Test: Flatpak-Build funktioniert + Portal-Netzwerkmonitor aktiv
- [ ] #5 cargo clippy --all -- -D warnings pass
<!-- DOD:END -->
