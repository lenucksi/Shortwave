---
id: SW-14
title: "CLI Tool: shortwave-script"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: tooling
dependencies:
  - SW-8
references:
  - src/discovery/runner.rs
  - src/discovery/engine.rs
  - src/discovery/registry.rs
  - 'https://docs.rs/clap/latest/clap/'
  - tools/rhai-runner/
priority: low
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ein separates CLI-Binary `shortwave-script`, das Rhai-Scripts
auch ohne GUI ausführen kann. Das ist essentiell für:
- Externes Testen von Scripts während der Entwicklung
- CI/CD: Scripts in GitHub/GitLab CI testen
- Debugging: Schrittweise Script-Entwicklung ohne App-Neustart
- User: "Probiere mein Script aus bevor ich es in die App lege"

Commands:
- `shortwave-script run <path> [--timeout N]` → Execute Script, print JSON
- `shortwave-script list` → List verfügbare Provider in datadir/userdir
- `shortwave-script test <path> [--verbose]` → Execute mit Debug-Log
- `shortwave-script validate <path>` → Syntax-Check + Output-Validierung
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `shortwave-script run somafm.rhai` gibt JSON auf stdout aus
- [ ] #2 `shortwave-script list` zeigt verfügbare Provider
- [ ] #3 `shortwave-script test somafm.rhai --verbose` zeigt Debug-Ausgabe
- [ ] #4 `shortwave-script validate --strict somafm.rhai` validiert JSON-Output-Schema
- [ ] #5 CLI hat `--help` + man page
- [ ] #6 `cargo build` produziert korrektes Binary
- [ ] #7 CLI Error-Handling: non-zero exit bei Fehlern
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **Cargo.toml**: `[[bin]]` Section für shortwave-script:
   ```toml
   [[bin]]
   name = "shortwave-script"
   path = "src/bin/shortwave-script.rs"
   ```

2. **Optional**: `clap = { version = "4", features = ["derive"] }` als dev-dep
   oder optional feature (für CLI-Parsing)

3. **src/bin/shortwave-script.rs**:
   ```rust
   #[derive(Parser)]
   enum Cli {
       Run { path: PathBuf, timeout: Option<u64> },
       List,
       Test { path: PathBuf, verbose: bool },
       Validate { path: PathBuf, strict: bool },
   }
   ```

4. **List-Command**:
   - Nutzt ProviderRegistry mit datadir/userdir
   - Listet Provider mit Name, Pfad, Status (bundled/user)

5. **Run-Command**:
   - Engine erstellen (mit http_get etc.)
   - Script laden + ausführen
   - JSON auf stdout
   - Exit-Code: 0 = OK, 1 = Error

6. **Validate-Command**:
   - Script syntax-check (Rhai Engine::compile)
   - Ausführen + Output als JSON parsen
   - Mit `--strict`: Prüft ob alle Pflichtfelder (name, stream_url) gesetzt
   - Reports fehlende Felder

7. **Build-Integration**:
   - `src/meson.build` anpassen für zweites Binary
   - `data/de.haeckerfelix.Shortwave.gresource.xml.in` (unwahrscheinlich nötig)
   - Flatpak manifest: binary muss mit installiert werden

8. **Man-Page**: `data/shortwave-script.1` (optional, spätere Runde)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 `cargo run --bin shortwave-script -- run data/providers/somafm.rhai` liefert JSON
- [ ] #3 `cargo run --bin shortwave-script -- list` zeigt Provider
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 Branch gemerged in main
<!-- DOD:END -->
