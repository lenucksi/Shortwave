---
id: SW-14
title: "CLI Tool: shortwave-script"
status: Pending
assignee: []
created_date: 2026-05-24
updated_date: 2026-05-26 20:55
labels: []
milestone: tooling
dependencies:
  - SW-8
references:
  - src/discovery/runner.rs
  - src/discovery/engine.rs
  - src/discovery/registry.rs
  - https://docs.rs/clap/latest/clap/
  - tools/rhai-runner/
priority: medium
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ein separates CLI-Binary `shortwave-script`, das Rhai-Scripts auch ohne GUI ausführen kann. Essentiell für Script-Entwicklung, Testing und CI/CD.\n\nCommands:\n- `shortwave-script run <path> [--timeout N] [--pretty]` → Execute Script, print JSON (oder human-readable mit --pretty)\n- `shortwave-script list` → List verfügbare Provider\n- `shortwave-script validate <path>` → Syntax-Check + Output-Validierung\n\nDer `--pretty`-Modus gibt eine menschenlesbare Stationen-Liste aus statt rohem JSON: zeigt Name, Stream-URL(s), Codec, Bitrate, TLS-Status, Tags, Land.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `shortwave-script list` zeigt verfügbare Provider
- [ ] #2 `shortwave-script test somafm.rhai --verbose` zeigt Debug-Ausgabe
- [ ] #3 `shortwave-script validate --strict somafm.rhai` validiert JSON-Output-Schema
- [ ] #4 CLI hat `--help` + man page
- [ ] #5 `cargo build` produziert korrektes Binary
- [ ] #6 CLI Error-Handling: non-zero exit bei Fehlern
- [ ] #7 shortwave-script run somafm.rhai gibt JSON auf stdout aus
- [ ] #8 shortwave-script run somafm.rhai --pretty zeigt human-readable Stationen-Liste
- [ ] #9 shortwave-script list zeigt verfügbare Provider
- [ ] #10 shortwave-script validate somafm.rhai validiert JSON-Output-Schema
- [ ] #11 CLI hat --help
- [ ] #12 cargo build produziert korrektes Binary
- [ ] #13 CLI Error-Handling: non-zero exit bei Fehlern
<!-- AC:END -->



## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Cargo.toml: [[bin]] section + clap = { version = "4", features = ["derive"] }\n2. src/bin/shortwave-script.rs mit clap CLI\n3. Run-Command: engine erstellen, Script ausführen, JSON auf stdout\n4. --pretty Flag: parsed DiscoveryResult serialisieren als menschenlesbare Tabelle (Name, URL, Codec, Bitrate, TLS)\n5. List-Command: ProviderRegistry scannt datadir/userdir\n6. Validate-Command: compile + run + schema-check\n7. Error-Handling: non-zero exit, Fehler auf stderr
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
