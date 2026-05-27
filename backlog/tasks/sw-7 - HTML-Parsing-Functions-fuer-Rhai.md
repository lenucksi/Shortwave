---
id: SW-7
title: HTML Parsing Functions für Rhai (scraper crate)
status: Pending
assignee: []
created_date: 2026-05-24
updated_date: 2026-05-26 20:44
labels: []
milestone: engine-core
dependencies:
  - SW-6
references:
  - https://docs.rs/scraper/latest/scraper/
  - https://docs.rs/rhai/latest/rhai/
  - src/discovery/engine.rs
  - src/discovery/types.rs
  - Cargo.toml
priority: high
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
HTML-CSS-Selektion als Engine-Funktion: html_select(html, css) parst HTML via scraper crate, wendet CSS-Selektor an, gibt JSON-String-Array der gematchten Nodes zurueck. Script ruft json_parse() auf das Resultat -> arbeitet mit plain Rhai Dynamic maps/arrays (keine Custom Types, kein HtmlDoc/NodeRef).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `html_attr()` extrahiert beliebiges Attribut aus Node
- [ ] #2 `html_text()` extrahiert sichtbaren Text aus Node
- [ ] #3 `html_nodes()` erlaubt Sub-Selektionen auf bereits gematchten Nodes
- [ ] #4 Chaining funktioniert: `html_text(html_select(doc, "h1")[0])`
- [ ] #5 Edge Cases: invalides HTML, leerer String, nicht-existenter Selektor
- [ ] #6 Unit-Tests: CSS-Selektoren, Attribut-Extraktion, Document-Handling
- [ ] #7 `cargo clippy --all -- -D warnings` pass
- [ ] #8 html_select(html, css) -> JSON-String-Array via scraper + CSS-Selektor
- [ ] #9 Jeder Node im JSON hat: tag, attrs (Map), text (String), html (String)
- [ ] #10 json_parse(html_select(...)) -> Dynamic Array -> Zugriff auf [0].attrs["href"]
- [ ] #11 Edge Cases: invalides HTML, leerer String, nicht-existenter Selektor -> leeres Array, kein Crash
- [ ] #12 Unit-Tests: basic selection, attribute extraction, chaining mit json_parse
- [ ] #13 cargo clippy --all -- -D warnings pass
<!-- AC:END -->





## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Cargo.toml: scraper = "0.22" hinzufuegen\n2. src/discovery/engine.rs: html_select(html_str, css) registrieren — scraper::Html::parse_document + Selector::parse + select, serialisiere Nodes als JSON { tag, attrs, text, html }\n3. Keine Custom Types. Pattern direkt aus tools/rhai-runner/src/main.rs\n4. Unit-Tests: html_select_basic, html_select_attr, html_select_invalid, html_select_empty, html_select_chaining
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- TBD
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Unit-Tests in src/discovery/engine.rs
- [ ] #3 `cargo test discovery` läuft grün
- [ ] #4 `cargo clippy --all -- -D warnings` pass
- [ ] #5 Branch gemerged in main
<!-- DOD:END -->
