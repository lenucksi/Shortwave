---
id: DRAFT-1
title: ui für extraktoren
status: Draft
assignee: []
created_date: 2026-05-28 09:56
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
die alte suche sollte aus dem main add dialog raus
stattdessen sollten die ergebnisse der neuen geladen werden jeweils in sektionen pro skript getrennt, typ hrule zwischen den ergebnissen. darin kann man dann locally filtern. 
dann da - wie die ui für batch delete - selektieren können was geadded werden soll.
alternativ auf station klicken ansicht der daten wie der dialog der in den bereits geaddeten stations die infos anzeigt und dann da statt play einen add button einbauen. oder zusätzlich zu add. so dass man auch temporär reinhören kann ohne zu added. und im bereits geaddeten view kann dann der add button ausgeblendet werden.

der userflow für die api-suche bleibt dann natürlich leider offen.
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 cargo test passes (all tests)
- [ ] #2 cargo clippy --all -- -D warnings clean
- [ ] #3 Test coverage added where possible (pure functions, parsers, serialization)
- [ ] #4 Branch gemerged in lokales main (oder PR-ready falls remote tot)
<!-- DOD:END -->