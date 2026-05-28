---
id: SW-9
title: WebradioDB API Provider Script
status: Archived
assignee: []
created_date: 2026-05-24
updated_date: 2026-05-24
labels: []
milestone: provider-system
dependencies:
  - SW-8
references:
  - https://github.com/jcorporation/webradiodb
  - https://jcorporation.github.io/webradiodb/
  - https://gitlab.gnome.org/World/Shortwave/-/work_items/717
  - src/discovery/runner.rs
  - src/discovery/types.rs
  - src/discovery/engine.rs
  - data/providers/
priority: high
ordinal: 9000
---
## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der erste echte Discovery-Provider: ein Rhai-Script, das die
WebradioDB JSON-API anspricht und Stations-Daten ins
DiscoveryResult-Format konvertiert.

WebradioDB ist der alternative Service aus GitLab Issue #717.
Die API liefert JSON → keine HTML-Parsing nötig, nur HTTP + JSON Mapping.

Das Script wird als bundled Provider in `$DATADIR/shortwave/providers/`
ausgeliefert.

API: `GET https://api.webradiodb.com/v1/stations` (Beispiel-URL, muss
anhand der tatsächlichen WebradioDB API Doku angepasst werden).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Rhai-Script existiert in `data/providers/webradiodb.rhai`
- [ ] #2 Script ruft WebradioDB-API via `http_get_json()` auf
- [ ] #3 Script mapped API-Response auf `StationData`-Format
- [ ] #4 Script gibt valides `DiscoveryResult`-JSON aus
- [ ] #5 Script läuft durch (max 30s)
- [ ] #6 Fehlerbehandlung: API-down → leeres Result, kein Crash
- [ ] #7 `shortwave-script run data/providers/webradiodb.rhai` liefert Stations-Liste
- [ ] #8 Provider wird von Registry erkannt (Datei in datadir)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. WebradioDB API Doku studieren (GitHub README / Webseite)

2. **data/providers/webradiodb.rhai** schreiben:
   ```rust
   // Discovery-Provider: WebradioDB
   // Description: Community-kuratierte Radios, Alternative zu Radio-Browser.info
   // URL: https://jcorporation.github.io/webradiodb/

   let response = http_get_json("https://api.webradiodb.com/v1/stations?limit=100");

   let stations = [];
   for entry in response {
       let stream_urls = [];
       if entry.url != "" {
           stream_urls.push(#{
               "url": entry.url,
               "codec": entry.codec,
               "bitrate": entry.bitrate,
               "tls": entry.url.starts_with("https")
           });
       }

       stations.push(#{
           "name": entry.name,
           "stream_url": entry.url,
           "stream_urls": stream_urls,
           "homepage": entry.homepage,
           "icon_url": entry.favicon,
           "tags": entry.tags,
           "country": entry.country,
           "language": entry.language
       });
   }

   json_stringify(#{
       "provider": "WebradioDB",
       "stations": stations
   })
   ```

3. **Test-Script**: Mit `http_get_json` gegen echte API testen
   (entweder via `shortwave-script run` oder direkt gegen API)

4. **Fallback**: Bei API-Ausfall leerer Stations-Array, kein panic
   - `try http_get_json(...)` oder Error-Handling in Rhai
   - Rhai hat `try`/`catch` für Error-Handling

5. **FEEDBACK-LOOP**: Script muss tatsächlich funktionieren — Test gegen
   die echte API ist Teil der AC
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 `shortwave-script run data/providers/webradiodb.rhai` liefert Stations-JSON
- [ ] #3 Alle Stations haben name + stream_url gesetzt
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 Branch gemerged in main
<!-- DOD:END -->