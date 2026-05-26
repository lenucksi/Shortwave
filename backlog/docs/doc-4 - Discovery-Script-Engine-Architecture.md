# Discovery Script Engine — Architecture &amp; Decision Record

## Problem

Shortwave hängt aktuell hart an `radio-browser.info` als einziger Quelle für
Stations-Discovery. Der Service ist oft unzuverlässig, DNS-basiertes Failover
ist fragile, und neue Quellen (WebradioDB, SomaFM, API-X) können nicht ohne
Code-Änderungen angebunden werden.

## Lösung

Ein **Discovery Script Engine** — ein eingebetteter Rhai-Interpreter, der
"Extraction Scripts" ausführt. Jedes Script parst eine Website oder API
und gibt standardisiertes JSON mit Stations-Listen zurück.

```
┌──────────────────────────────────────────────────────┐
│  Shortwave App                                        │
│  ┌──────────────────┐   ┌──────────────────────────┐ │
│  │  Discovery Page   │   │  Import Results Dialog   │ │
│  └────────┬─────────┘   └──────────┬───────────────┘ │
│           │                        │                 │
│  ┌────────▼────────────────────────▼──────────────┐ │
│  │  discovery::registry                           │ │
│  │  - scannt ~/.local/share/shortwave/providers/  │ │
│  │  - scannt $DATADIR/shortwave/providers/         │ │
│  │  - listet verfügbare Provider                   │ │
│  └────────┬───────────────────────────────────────┘ │
│           │                                          │
│  ┌────────▼───────────────────────────────────────┐ │
│  │  discovery::runner                             │ │
│  │  - lädt .rhai Datei                            │ │
│  │  - executed in Rhai Engine                     │ │
│  │  - parsed JSON Output in StationData Vec       │ │
│  │  - timeout + error handling                    │ │
│  └────────┬───────────────────────────────────────┘ │
│           │                                          │
│  ┌────────▼───────────────────────────────────────┐ │
│  │  discovery::engine                             │ │
│  │  - Rhai Engine + registrierte Funktionen       │ │
│  │  - http_get, html_parse, json_parse, ...       │ │
│  │  - Sandboxing (max ops, max calls)             │ │
│  └────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘

     Extern testbar:
     $ shortwave-script run somafm.rhai
     → JSON-Array an Stationen auf stdout
```

## Warum Rhai?

| Kriterium | Rhai | mlua (Lua) | rquickjs (JS) |
|-----------|------|-------------|---------------|
| Sprache | Rust (kein C) | C (vendored) | C (vendored) |
| Binary Impact | ~300-600 KB | ~1-2 MB | ~1-2 MB |
| Sandboxing | ✅ default | ⚠️ manuell | ⚠️ manuell |
| Extern testbar | `rhai-runner` | `lua` CLI | `qjs` CLI |
| Flathub | ✅ trivial | ✅ mit C build | ✅ mit C build |
| HTTP/IO | ❌ nicht built-in | ❌ nicht built-in | ❌ nicht built-in |
| JSON | ✅ serde feature | ✅ serde feature | ✅ native |
| Async | sync (bridgebar) | ✅ builtin | ✅ Promises |

**Entscheidung: Rhai** weil:
- Pure Rust = kein C-Compiler im Flathub-Build nötig
- Sandboxing *by default*: Scripts haben keinerlei OS-Zugriff
- Wir kontrollieren exakt, welche Funktionen registriert werden
- Minimale binary impact (Shortwave ist bereits ~8 MB)
- Extern testbar: `rhai-runner script.rhai` oder eigener CLI-Shim

## Script-Kontrakt

Jedes Discovery-Script muss ein JSON-Objekt auf stdout ausgeben:

```json
{
  "provider": "SomaFM",
  "stations": [
    {
      "name": "Groove Salad",
      "stream_url": "https://somafm.com/groovesalad.pls",
      "stream_urls": [
        {
          "url": "https://somafm.com/groovesalad.pls",
          "codec": "MP3",
          "bitrate": 256,
          "tls": true
        },
        {
          "url": "https://somafm.com/groovesalad128.pls",
          "codec": "MP3",
          "bitrate": 128,
          "tls": true
        },
        {
          "url": "https://somafm.com/groovesalad130.pls",
          "codec": "AAC",
          "bitrate": 128,
          "tls": true
        }
      ],
      "homepage": "https://somafm.com/groovesalad/",
      "icon_url": "https://somafm.com/logos/120/groovesalad120.png",
      "tags": "ambient,chill,electronic",
      "country": "US",
      "language": "en"
    }
  ]
}
```

Minimale Felder: `name` + `stream_url`. Alles andere optional.
Jeder Eintrag in `stream_urls` kann Metadaten (`codec`, `bitrate`, `tls`)
mithaben — der Player kann damit den passenden Stream wählen.

## Rhai Engine API (registrierte Funktionen)

### HTTP
```rust
http_get(url: &str) -> String
http_get_json(url: &str) -> Dynamic   // auto-parsed JSON
http_get_with_timeout(url, secs) -> String
```

Implementation: wrappt `reqwest::blocking::get()` mit Configurable Timeout.

### HTML (via `scraper` crate)
```rust
html_parse(html: &str) -> Dynamic         // → Document
html_select(doc, css: &str) -> Array      // → Vec<Node>
html_attr(node, attr: &str) -> String
html_text(node) -> String
html_nodes(node, css: &str) -> Array      // Sub-Selektor
```

### JSON
```rust
json_parse(s: &str) -> Dynamic
json_stringify(val) -> String
```

### Utility
```rust
log(msg: &str)
sleep(ms: i64)
```

## Provider Directories

| Pfad | Typ |
|------|-----|
| `$DATADIR/shortwave/providers/*.rhai` | Bundled (vom Paket) |
| `~/.local/share/shortwave/providers/*.rhai` | User-added |

Registry scanned beide bei Startup. Jede `.rhai` Datei = ein Provider.

Die erste Zeile oder ein Metadata-Header im Script kann Titel + Description
definieren (optional; fallback = filename).

```
// Discovery-Provider: SomaFM
// Description: Scraped von somafm.com — 30+ Channels

let html = http_get("https://somafm.com/");
...
```

## Was sich ändert

### Neu: `src/discovery/`
```
src/discovery/
├── mod.rs              # pub use
├── types.rs            # DiscoveryResult, StationData (serde)
├── engine.rs           # Rhai Engine + Funktionen registrieren
├── provider.rs         # ProviderConfig, DiscoveryProvider
├── registry.rs         # Provider scan + management
├── runner.rs           # Script execution + output parsing
```

### Neu: `src/bin/shortwave-script.rs`
CLI-Tool zum Testen von Scripts ohne GUI.

### Geändert: `Cargo.toml`
Neue Dependencies:
- `rhai` (mit serde feature)
- `scraper` (HTML parsing für CSS-Selektoren)

Optional für CLI:
- `clap` (argument parsing)

### Geändert: `src/ui/pages/search_page.rs`
Discovery Sources als zusätzliche Stations-Quelle.

### Geändert: `data/gtk/`
Neue UI Templates:
- `discovery_source_row.ui`
- `discovery_results_dialog.ui`

## Milestones

| Milestone | Tasks | Ergebnis |
|-----------|-------|----------|
| **engine-core** | SW-6, SW-7 | Rhai Engine + HTTP + HTML Funktionen |
| **provider-system** | SW-8, SW-9, SW-10 | Registry + Runner + erste Scripts |
| **ui-integration** | SW-11, SW-12 | Discovery Sources UI + Import Dialog |
| **tooling** | SW-13, SW-14, SW-15 | CLI + Custom Provider Mgmt + Docs |

## Out of Scope (für diese Runde)

- **Auto-Update** von bundled Scripts (später via Flatpak updates)
- **Script Markets** / Remote-Repositories
- **GraphQL** oder andere Protokolle (reines HTTP reicht)
- **WASM**-basierte Scripts (zu heavy, zu komplex)
- **Python/JS** als Scriptsprache (zu schwer embeddable)

## Anmerkungen zur Sicherheit

Rhai sandboxt standardmäßig:
- Kein Filesystem-Zugriff
- Kein Network-Zugriff (nur über registrierte `http_get`)
- `Engine::set_max_operations()` → verhindert Endlosschleifen
- `Engine::set_max_call_levels()` → verhindert Rekursions-Angriffe
- `Engine::set_max_strings()` → verhindert Memory-Exhaustion

Ein Discovery-Script kann *nur* tun, was wir explizit erlauben.
