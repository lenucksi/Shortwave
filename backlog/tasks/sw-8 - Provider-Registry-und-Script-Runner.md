---
id: SW-8
title: "Provider Registry + Script Runner (Async Execution)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: provider-system
dependencies:
  - SW-6
  - SW-7
references:
  - src/discovery/engine.rs
  - src/discovery/types.rs
  - src/discovery/provider.rs
  - src/discovery/registry.rs
  - src/discovery/runner.rs
  - src/app.rs
  - 'https://docs.rs/rhai/latest/rhai/engine/struct.Engine.html#method.run_file'
  - tools/rhai-runner/src/main.rs
priority: high
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Provider Registry + Script Runner — das Herzstück der Discovery Engine.

Die Registry:
- Scannt zwei Verzeichnisse nach `.rhai` Dateien:
  - System: `$DATADIR/shortwave/providers/` (bundled)
  - User: `~/.local/share/shortwave/providers/` (custom)
- Liefert eine Liste von `DiscoveryProvider`-Configs (name, path, enabled)
- Erlaubt Enable/Disable pro Provider

Der Runner:
- Nimmt einen Provider (Pfad zur .rhai Datei) + die konfigurierte Engine
- Führt das Script aus (mit Timeout + Sandboxing)
- Parst stdout/return value als JSON → `Vec<StationData>` oder `DiscoveryResult`
- Gibt strukturierte Errors zurück (Parse-Error, Timeout, Runtime-Error)
- Läuft asynchron: `async fn run_provider(provider) -> Result<DiscoveryResult>`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Registry scanned `$DATADIR/providers/` + `~/.local/share/shortwave/providers/`
- [ ] #2 Registry ignoriert Non-`.rhai` Dateien
- [ ] #3 `DiscoveryProvider` hat name, path, enabled, description
- [ ] #4 Provider-Name fallback: filename ohne .rhai (wenn kein Metadata-Header)
- [ ] #5 Runner führt `.rhai` Script aus und parst JSON-Output
- [ ] #6 Runner timeout bei >30s Script-Laufzeit
- [ ] #7 Runner Error-Handling: Parse-Error → Result::Err, Runtime-Error → Result::Err
- [ ] #8 Runner injected Discovery-Result als `DiscoveryResult` Struct
- [ ] #9 `SwApplication` hat eine `discovery_registry` Instanz (oder via Singleton)
- [ ] #10 Unit-Tests für Registry (scannen, filtern, fehlende dirs)
- [ ] #11 Integration-Tests für Runner (echte .rhai files mit mock-HTTP)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **src/discovery/provider.rs**:
   ```rust
   pub struct DiscoveryProvider {
       pub id: String,          // "somafm"
       pub name: String,        // "SomaFM"
       pub description: String, // "Scraped von somafm.com"
       pub script_path: PathBuf,
       pub enabled: bool,
   }
   ```

2. **src/discovery/registry.rs**:
   - `ProviderRegistry` struct
   - `fn new(datadir: &Path, userdir: &Path) -> Self`
   - `fn scan() -> Vec<DiscoveryProvider>`
   - `fn providers() -> &[DiscoveryProvider]`
   - `fn enable(id: &str)` / `fn disable(id: &str)`
   - .rhai detection: filename.ends_with(".rhai")
   - Metadata-Header parsen: `// Discovery-Provider: Name` + `// Description: ...`
   - Sortierung: zuerst bundled, dann user (user überschreibt bundled bei gleichem id)

3. **src/discovery/runner.rs**:
   - `pub fn run_provider(engine: &Engine, provider: &DiscoveryProvider) -> Result<DiscoveryResult>`
   - Intern: `engine.run_file::<Dynamic>(path)` → JSON stringify → serde_json parse
   - `pub async fn run_provider_async(...)` → `spawn_blocking` für Rhai execution
   - Timeout via `tokio::time::timeout(Duration::from_secs(30), ...)`
   - Error-Types: `RunnerError { kind: ParseError | Timeout | RuntimeError, message }`

4. **Integration in SwApplication**:
   - `app.rs`: `ProviderRegistry` als Feld
   - Bei `startup()`: registry initialisieren mit datadir/userdir
   - Option: Refresh-Button im UI

5. **src/bin/shortwave-script.rs** (rudimentär für Tests):
   - `shortwave-script run <path>` → execute + print JSON
   - `shortwave-script list` → list verfügbare Provider
   - (CLI wird in SW-13 ausgebaut)

6. **Mock-HTTP für Tests**:
   - Eigener Test-Server (mini HTTP Server) oder Mock-Objekte
   - Oder: `wiremock` oder `httpmock`-ähnliches Setup
   - Oder: Reale HTTP-Calls gegen stabile Endpunkte (wiki.archlinux.org o.ä.)
   - Entscheidung: reale HTTP-Calls + Timeout (einfach, robust genug)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Unit-Tests + Integration-Tests pass
- [ ] #3 `cargo test discovery` läuft grün
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 Branch gemerged in main
<!-- DOD:END -->
