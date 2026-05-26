---
id: SW-13
title: "Radio Browser API Provider Script"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: provider-system
dependencies:
  - SW-8
references:
  - 'https://www.radio-browser.info/'
  - src/api/client.rs
  - src/api/station_request.rs
  - src/api/mod.rs
  - data/providers/
priority: medium
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ein Rhai-Script, das die bestehende Radio-Browser.info API als
Discovery-Provider verfügbar macht. Damit wird der alte,
hart-verdrahtete API-Zugriff (src/api/) durch einen Script-basierten
Provider ersetzt — oder parallel dazu existieren.

Das Script mapped die radio-browser.info JSON-Response auf das
DiscoveryResult-Format. Es nutzt `http_get_json()` und
ist ein reines JSON→JSON-Transformation-Script.

Dies ist der "Exit-Strategy"-Task: Sobald alle Discovery-Provider
als Scripts vorliegen, kann der alte src/api/ Code langsam
deprecated werden.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Rhai-Script existiert in `data/providers/radio-browser.rhai`
- [ ] #2 Script ruft `all.api.radio-browser.info/json/stations` API auf
- [ ] #3 Script unterstützt Parameter: tag, limit, order (votes/random)
- [ ] #4 Script mapped API-Response auf StationData inkl. aller Felder
- [ ] #5 Script gibt valides DiscoveryResult-JSON aus
- [ ] #6 `shortwave-script run data/providers/radio-browser.rhai` liefert Stations
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **data/providers/radio-browser.rhai** schreiben:
   ```rust
   // Discovery-Provider: Radio Browser
   // Description: radio-browser.info — die größte öffentliche Radio-Datenbank
   // URL: https://www.radio-browser.info/

   // Standard-Filter: beliebte Stationen, HTTPS, nicht broken
   let url = "https://all.api.radio-browser.info/json/stations/search?" +
       "order=votes&reverse=true&limit=100&hidebroken=true&is_https=true";

   let response = http_get_json(url);
   let stations = [];

   for entry in response {
       stations.push(#{
           "name": entry.name,
           "stream_url": entry.url_resolved ?? entry.url,
           "homepage": entry.homepage,
           "icon_url": entry.favicon,
           "tags": entry.tags,
           "country": entry.country,
           "language": entry.language,
           "codec": entry.codec,
           "bitrate": entry.bitrate,
           "countrycode": entry.countrycode
       });
   }

   json_stringify(#{
       "provider": "Radio Browser",
       "stations": stations
   })
   ```

2. **Feld-Mapping**:
   - `url_resolved` prioritär, fallback auf `url`
   - `StationMetadata`-Felder auf `StationData` mappen
   - Zusätzliche Felder aus StationMetadata können via `tags` oder
     eigenem Map-Eintrag transportiert werden

3. **Persistenz**: Später könnte das Script die DNS-basierte
   Server-Discovery aus src/api/client.rs ersetzen — aktuell
   hardcoded `all.api.radio-browser.info`
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 `shortwave-script run data/providers/radio-browser.rhai` liefert Stations-JSON
- [ ] #3 `cargo clippy --all -- -D warnings` pass
- [ ] #4 Branch gemerged in main
<!-- DOD:END -->
