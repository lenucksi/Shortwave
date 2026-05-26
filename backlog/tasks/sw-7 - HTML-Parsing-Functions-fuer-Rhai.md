---
id: SW-7
title: "HTML Parsing Functions für Rhai (scraper crate)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels: []
milestone: engine-core
dependencies:
  - SW-6
references:
  - 'https://docs.rs/scraper/latest/scraper/'
  - 'https://docs.rs/rhai/latest/rhai/'
  - src/discovery/engine.rs
  - src/discovery/types.rs
  - Cargo.toml
priority: high
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Discovery-Scripts müssen Websites parsen können (z.B. SomaFM, Radio-Specific).
Dazu wird das `scraper` crate als HTML-Parser + CSS-Selektor-Engine
in die Rhai Engine eingebunden.

Registrierte Funktionen:
- `html_parse(html: &str)` → Dynamic (Document als opaque Handle)
- `html_select(doc, css: &str)` → Array (NodeList)
- `html_attr(node, attr: &str)` → String
- `html_text(node)` → String
- `html_nodes(node, css: &str)` → Array (Sub-Selektor auf Node)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `html_parse()` erzeugt ein Document aus HTML-String
- [ ] #2 `html_select()` liefert Node-Array via CSS-Selektor
- [ ] #3 `html_attr()` extrahiert beliebiges Attribut aus Node
- [ ] #4 `html_text()` extrahiert sichtbaren Text aus Node
- [ ] #5 `html_nodes()` erlaubt Sub-Selektionen auf bereits gematchten Nodes
- [ ] #6 Chaining funktioniert: `html_text(html_select(doc, "h1")[0])`
- [ ] #7 Edge Cases: invalides HTML, leerer String, nicht-existenter Selektor
- [ ] #8 Unit-Tests: CSS-Selektoren, Attribut-Extraktion, Document-Handling
- [ ] #9 `cargo clippy --all -- -D warnings` pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. **Cargo.toml**: `scraper = "0.22"` hinzufügen

2. **src/discovery/engine.rs** erweitern:
   - `HtmlDoc` als Rhai-custom-type (struct mit scraper::Html)
   - `NodeRef` als Rhai-custom-type (geparster Node)
   - `html_parse(html)` → `Dynamic(HtmlDoc)`
   - `html_select(doc, css)` → `Vec<Dynamic(NodeRef)>`
   - `html_attr(node, attr)` → `String`
   - `html_text(node)` → `String`
   - `html_nodes(node, css)` → `Vec<Dynamic(NodeRef)>`

3. **Custom Type Implementierung**:
   - `HtmlDoc` wrapt `scraper::Html` → registriert als `"html_doc"`
   - `NodeRef` wrapt `scraper::ElementRef` → registriert als `"html_node"`
   - Beide implementieren `rhai::CustomType` für Type-Checks zur Laufzeit

4. **Unit-Tests**:
   - `test_html_parse_select`: "&lt;ul>&lt;li>A&lt;/li>&lt;li>B&lt;/li>&lt;/ul>" mit "li" → 2 Nodes
   - `test_html_attr`: "&lt;a href='x'>y&lt;/a>" → href = "x"
   - `test_html_text`: Text-Extraktion mit/ohne Child-Elements
   - `test_html_invalid`: "&lt;p>hello" → immer noch parsbares Document
   - `test_html_empty`: "" → leeres Document
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
