---
id: SW-10
title: SomaFM Scraper Provider Script
status: Archived
assignee: []
created_date: 2026-05-24
updated_date: 2026-05-24
labels: []
milestone: provider-system
dependencies:
  - SW-8
references:
  - https://somafm.com/
  - https://somafm.com/#alpha
  - src/discovery/runner.rs
  - src/discovery/types.rs
  - src/discovery/engine.rs
  - data/providers/
  - tools/rhai-runner/
  - data/providers/somafm.rhai
priority: medium
ordinal: 10000
---
## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Zweiter Discovery-Provider: ein Rhai-Script, das die SomaFM-Website
per HTML-Scraping parst und alle Channels + deren PLS-URLs + Icons
als Stations-Liste zurückgibt.

SomaFM hat keine öffentliche API — also Website-Scraping via
`html_parse()` + `html_select()`.

Die SomaFM-Struktur:
- Channel-Liste auf https://somafm.com/
- Jeder Channel hat ein `<div>` mit Klasse `channel` (oder ähnlich)
- Channel-Name, Listen-Link (.pls), Icon-Bild, Beschreibung
- PLS-URLs folgen Muster: `https://somafm.com/<channel><bitrate>.pls`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Rhai-Script existiert in `data/providers/somafm.rhai`
- [ ] #2 Script fetcht https://somafm.com/ via `http_get()`
- [ ] #3 Script parsed HTML via `html_parse()` + CSS-Selektoren
- [ ] #4 Script extrahiert >=20 Channels (SomaFM hat ~30)
- [ ] #5 Jeder Channel hat name + stream_url + icon_url + homepage
- [ ] #6 stream_url zeigt auf PLS-URL (z.B. ...256.pls)
- [ ] #7 Script gibt valides `DiscoveryResult`-JSON aus
- [ ] #8 Script läuft durch (max 30s)
- [ ] #9 Fehlerbehandlung: Seite geändert → leeres Result, kein Crash
- [ ] #10 `shortwave-script run data/providers/somafm.rhai` liefert Stations-JSON
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. SomaFM-HTML-Struktur analysieren (View Page Source auf somafm.com)

   Struktur der Hauptseite:
   - Channel-Links: `a[href^='/player24/station/']` mit `<img>` child
   - Channel-Name im `alt`-Attribut des img: "Groove Salad: ambient/electronic ..."
   - Icon-URL im `src` des img: "/logos/120/groovesalad120.png"

   SomaFM-Naming-Convention für Streams:
   - `{slug}.pls` = MP3 256k (SSL)
   - `{slug}128.pls` = MP3 128k (SSL)
   - `{slug}130.pls` = AAC 128k (SSL)
   - `{slug}64.pls` = AAC 64k (SSL)
   - `{slug}32.pls` = AAC 32k (SSL)
   - `/nossl/{slug}.pls` = MP3 256k (non-SSL), etc.

2. **data/providers/somafm.rhai** schreiben:
   ```rust
   // Discovery-Provider: SomaFM
   // Description: 30+ kuratierte Channels (ambient, electronic, indie, ...)

   let html = http_get("https://somafm.com/");
   let doc = html_parse(html);

   let links = html_select(doc, "a[href^='/player24/station/']");
   let seen = #{};
   let stations = [];

   // Alle bekannten Stream-Formate inkl. Metadaten
   let formats = [
       #{ "suffix": ".pls",   "codec": "MP3", "bitrate": 256, "tls": true,  "subdir": ""       },
       #{ "suffix": "128.pls","codec": "MP3", "bitrate": 128, "tls": true,  "subdir": ""       },
       #{ "suffix": "130.pls","codec": "AAC", "bitrate": 128, "tls": true,  "subdir": ""       },
       #{ "suffix": "64.pls", "codec": "AAC", "bitrate": 64,  "tls": true,  "subdir": ""       },
       #{ "suffix": "32.pls", "codec": "AAC", "bitrate": 32,  "tls": true,  "subdir": ""       },
       #{ "suffix": ".pls",   "codec": "MP3", "bitrate": 256, "tls": false, "subdir": "/nossl" },
       #{ "suffix": "128.pls","codec": "MP3", "bitrate": 128, "tls": false, "subdir": "/nossl" },
       #{ "suffix": "130.pls","codec": "AAC", "bitrate": 128, "tls": false, "subdir": "/nossl" },
       #{ "suffix": "64.pls", "codec": "AAC", "bitrate": 64,  "tls": false, "subdir": "/nossl" },
       #{ "suffix": "32.pls", "codec": "AAC", "bitrate": 32,  "tls": false, "subdir": "/nossl" },
   ];

   for link in links {
       let href = html_attr(link, "href");
       let slug = href.split("/")[3];

       if seen.has(slug) { continue; }
       seen[slug] = true;

       let img = html_select(link, "img");
       let name = "";
       let icon_url = "";
       if img.len() > 0 {
           let alt = html_attr(img[0], "alt");
           icon_url = "https://somafm.com" + html_attr(img[0], "src");
           name = alt.split(":")[0].trim();
       }

       let stream_urls = [];
       let primary_url = "";
       for f in formats {
           let url = "https://somafm.com" + f.subdir + "/" + slug + f.suffix;
           stream_urls.push(#{
               "url": url,
               "codec": f.codec,
               "bitrate": f.bitrate,
               "tls": f.tls
           });
           if primary_url == "" { primary_url = url; }
       }

       stations.push(#{
           "name": name,
           "stream_url": primary_url,
           "stream_urls": stream_urls,
           "homepage": "https://somafm.com/" + slug + "/",
           "icon_url": icon_url,
           "tags": "soma fm,electronic,independent",
           "country": "US",
           "language": "en"
       });
   }

   json_stringify(#{
       "provider": "SomaFM",
       "stations": stations
   })
   ```

3. **Selektor-Strategie**: `a[href^='/player24/station/']`
   erfasst alle Channel-Karten. Dedup via Slug (jede Station
   erscheint 2x: "all"-Sektion + Kategorie-Sektion).
   Channel-Name aus `img[alt]` per Splitten am Doppelpunkt.

4. **stream_urls mit Metadaten**: Jeder Eintrag hat
   `url`, `codec`, `bitrate`, `tls` — der Shortwave-Player kann
   damit intelligent den passenden Stream wählen (z.B. TLS > non-TLS,
   MP3 fallback wenn AAC nicht supported, etc.).

5. **PLS-URLs**: SomaFM liefert `.pls` URLs — der bestehende
   Playlist-Fetcher (SW-1) resolved diese automatisch.

6. **Test**: Gegen echte SomaFM-Seite
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 `shortwave-script run data/providers/somafm.rhai` liefert Stations-JSON
- [ ] #3 Mindestens 20 Stationen im Output
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 Branch gemerged in main
<!-- DOD:END -->
