---
id: SW-6
title: "Discovery Engine Core: Rhai Engine + Typen + HTTP"
status: Archived
assignee: []
created_date: 2026-05-24
updated_date: 2026-05-24
labels: []
milestone: engine-core
dependencies: []
references:
  - https://rhai.rs/
  - https://docs.rs/rhai/latest/rhai/
  - https://docs.rs/reqwest/latest/reqwest/
  - https://gitlab.gnome.org/World/Shortwave/-/work_items/717
  - src/discovery/
  - src/api/station_metadata.rs
  - Cargo.toml
  - tools/rhai-runner/
priority: high
ordinal: 6000
---
## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der fundamentale Baustein: Rhai Engine + das gemeinsame Datenaustausch-Format
für Discovery Scripts.

Dieser Task:
- Erstellt das `src/discovery/` Modul
- Definiert `StationData` und `DiscoveryResult` als serde-Structs
- Baut die Rhai Engine auf mit registrierten HTTP-Funktionen
- Fügt `rhai` (mit serde) + `reqwest` (blocking) als Dependencies

Die Engine muss:
- Rhai mit serde aktivieren (Map <-> Dynamic Konversion)
- `http_get(url)` → String registrieren (wrappt reqwest::blocking::get)
- `http_get_json(url)` → Dynamic registrieren (auto-parse JSON-Response)
- `json_parse(string)` → Dynamic registrieren
- `json_stringify(value)` → String registrieren
- Sandboxing: max_operations, max_call_levels, max_strings setzen
- Timeout-Handling für HTTP-Requests (default 10s)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `cargo build` kompiliert mit rhai + serde feature
- [ ] #2 `StationData` + `DiscoveryResult` serde-Structs existieren in `src/discovery/types.rs`
- [ ] #3 `discovery::engine::create()` gibt eine konfigurierte Rhai Engine zurück
- [ ] #4 `http_get(url)` funktioniert: liefert HTML/String body zurück
- [ ] #5 `http_get_json(url)` liefert geparsten Dynamic-Wert (Map wenn JSON-Object, Array wenn JSON-Array)
- [ ] #6 `json_parse` + `json_stringify` sind bidirektional konsistent (roundtrip)
- [ ] #7 Sandboxing: Script mit Endlosschleife wird nach N ops getötet
- [ ] #8 HTTP-Timeout: Script mit langsamem URL wird nach 10s abgebrochen
- [ ] #9 Unit-Tests für alle registrierten Funktionen
- [ ] #10 `cargo clippy --all -- -D warnings` pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **Cargo.toml**: `rhai = { version = "1", features = ["serde"] }` hinzufügen.
   Prüfen ob `reqwest` bereits blocking-fähig ist (ja, via `reqwest::blocking`).

2. **src/discovery/mod.rs**: Modul-Deklaration + `pub mod engine; pub mod types;`

3. **src/discovery/types.rs**:
   ```rust
   #[derive(Serialize, Deserialize, Debug, Clone)]
   pub struct StreamUrlInfo {
       pub url: String,
       pub codec: Option<String>,
       pub bitrate: Option<i32>,
       pub tls: Option<bool>,
   }

   #[derive(Serialize, Deserialize, Debug, Clone)]
   pub struct StationData {
       pub name: String,
       pub stream_url: String,
       #[serde(default)]
       pub stream_urls: Vec<StreamUrlInfo>,
       pub homepage: Option<String>,
       pub icon_url: Option<String>,
       pub tags: Option<String>,
       pub country: Option<String>,
       pub language: Option<String>,
   }

   #[derive(Serialize, Deserialize, Debug, Clone)]
   pub struct DiscoveryResult {
       pub provider: String,
       pub stations: Vec<StationData>,
   }
   ```

4. **src/discovery/engine.rs**:
   - `pub fn create() -> Engine`
   - `pub fn create_with_client(client: reqwest::blocking::Client) -> Engine`
   - Registriert `http_get`, `http_get_json`, `json_parse`, `json_stringify`
   - Setzt `engine.set_max_operations(500_000)`
   - Setzt `engine.set_max_call_levels(50)`
   - Setzt `engine.set_max_strings(10_000_000)`

5. **src/main.rs**: `mod discovery;` hinzufügen

6. **Unit-Tests** in `engine.rs` (#[cfg(test)]):
   - `test_http_get_json` (mock oder httpbin)
   - `test_json_roundtrip`
   - `test_sandbox_infinite_loop`
   - `test_station_data_serde`
   - `test_discovery_result_serde`
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Engine Unit-Tests in src/discovery/engine.rs
- [ ] #3 `cargo test discovery` läuft grün
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 Branch gemerged in main
<!-- DOD:END -->
