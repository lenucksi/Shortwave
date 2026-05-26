---
id: SW-16
title: "Dokumentation + Example Scripts"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: tooling
dependencies:
  - SW-9
  - SW-10
  - SW-14
references:
  - data/providers/
  - src/discovery/
  - README.md
  - CONTRIBUTING.md (falls existiert)
priority: low
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Dokumentation für das Discovery Script System — damit andere
Entwickler und Power-User eigene Provider-Scripts schreiben können.

Enthält:
- Developer Guide: "How to write a Discovery Script"
  - Rhai-Syntax-Crashkurs
  - Verfügbare Funktionen (http_get, html_parse, ...)
  - Output-Format (DiscoveryResult JSON)
  - Error-Handling (try/catch im Script)
  - Testing mit `shortwave-script test`
- API Reference für registrierte Rhai-Funktionen
- 3-4 Example Scripts in `data/examples/`:
  - SomaFM (variante vereinfacht)
  - WebradioDB (JSON API)
  - Radio Browser (JSON API)
- CONTRIBUTING-Update: "Adding a new Provider"
- README-Update: Feature-Erwähnung
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Developer Guide existiert als Markdown
- [ ] #2 API-Reference für alle registrierten Rhai-Funktionen
- [ ] #3 3 Example Scripts funktionieren mit `shortwave-script run`
- [ ] #4 README.md erwähnt Discovery Script System
- [ ] #5 Rhai-Syntax-Crashkurs für Neueinsteiger
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **docs/discovery-scripts.md**:
   - Rhai-Grundlagen (Variablen, Loops, Maps, try/catch)
   - Registrierte Funktionen (HTTP, HTML, JSON, Utility)
   - Output-Format (DiscoveryResult + StationData)
   - Best Practices (Error-Handling, Timeout, Selektoren)

2. **data/examples/README.md**:
   - Übersicht der Beispiel-Scripts
   - Ausführen mit `shortwave-script run examples/somafm-simple.rhai`

3. **data/examples/somafm-simple.rhai**: Vereinfachtes SomaFM-Script
   (auch als Tutorial)

4. **data/examples/webradiodb.rhai**: Kopie des SW-9 Scripts
   (oder symlink)

5. **data/examples/radio-browser.rhai**: Kopie des SW-13 Scripts

6. **README.md Update**:
   - "Discovery Scripts" in der Feature-Liste
   - Link zu docs/discovery-scripts.md
   - Quick-Start: Wie schreibe ich mein erstes Script?

7. **CONTRIBUTING.md Update** (falls vorhanden):
   - "Adding a new Discovery Provider" Schritt-für-Schritt
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Doku lesbar + vollständig
- [ ] #3 Example Scripts laufen mit CLI
- [ ] #4 Branch gemerged in main
<!-- DOD:END -->
