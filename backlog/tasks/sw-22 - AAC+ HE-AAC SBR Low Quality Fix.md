---
id: SW-22
title: "AAC+/HE-AAC SBR Low Quality Fix (#696)"
status: Pending
assignee: []
created_date: '2026-05-24'
updated_date: '2026-05-24'
labels:
  - bug
milestone: ""
dependencies: []
references:
  - 'https://gitlab.gnome.org/World/Shortwave/-/work_items/696'
  - src/audio/gstreamer_backend.rs
  - Cargo.toml
  - meson.build
  - build-aux/
  - 'http://live-aacplus-64.kexp.org/kexp64.aac'
  - 'http://ice.somafm.com/groovesalad-aac-64'
  - 'http://193.222.135.71/378'
priority: high
ordinal: 22000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
AAC+ (HE-AAC v1/v2) Streams mit Spectral Band Replication (SBR)
klingen in Shortwave dumpf/leise/muffig. Der gleiche Stream in VLC
klingt klar und hell. Ursache: uridecodebin wählt vermutlich einen
Decoder der SBR nicht korrekt dekodiert, oder die Caps-Verhandlung
endet auf der halben Samplerate (z.B. 22050Hz statt 44100/48000Hz).

Betroffene Stationen (Beispiele):
- KEXP 90.3 Seattle (AAC+): http://live-aacplus-64.kexp.org/kexp64.aac
- Radio 357: http://193.222.135.71/378
- SomaFM AAC-Stream: http://ice.somafm.com/groovesalad-aac-64

Dieser Task debuggt und fixt die AAC+ Decodierung im GStreamer-Pipeline.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] #1 Ursache identifiziert: welcher Decoder wird gewählt, welche Caps werden ausgehandelt
- [ ] #2 AAC+ (HE-AAC) Streams klingen klar/hell (wie VLC), nicht dumpf
- [ ] #3 AAC-LC (non-HE-AAC) Streams funktionieren weiterhin normal
- [ ] #4 MP3, Opus, OGG Vorbis Streams funktionieren weiterhin normal
- [ ] #5 Fallback-Strategie: wenn fdkaacdec nicht verfügbar → avdec_aac mit sbr=true
- [ ] #6 cargo fmt + cargo clippy --all -- -D warnings pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
### Subtask 1.1 — Debug: Decoder-Auswahl identifizieren
- Build mit GST_DEBUG=*AAC*:5,*decodebin*:5,*uridecodebin*:5
- uridecodebin's pad-added loggen: welche Caps werden ausgehandelt
- gst-launch-1.0-Tests außerhalb von Shortwave:
  ```
  # Aktuelles Verhalten simulieren
  gst-launch-1.0 uridecodebin uri=http://ice.somafm.com/groovesalad-aac-64 ! audioconvert ! autoaudiosink

  # Mit fdkaacdec
  gst-launch-1.0 souphttpsrc location=http://ice.somafm.com/groovesalad-aac-64 ! aacparse ! fdkaacdec ! audioconvert ! autoaudiosink
  ```
- Prüfen: fehlt SBR-Decodierung? caps sample rate zu niedrig?

### Subtask 1.2 — Decoder-Ranking via GstRegistry
In src/audio/gstreamer_backend.rs (init/module startup):
- gst::Registry::get() nach AAC-Decodern durchsuchen:
  - fdkaacdec (aus gst-plugins-bad) — bester HE-AAC Support
  - avdec_aac (aus gst-libav/FFmpeg) — gut mit sbr=true
  - faad (aus gst-plugins-good) — SBR-fähig
- fdkaacdec-Rank erhöhen: feature.set_rank(gst::Rank::Primary + 1)
  → uridecodebin autopluggt es bevorzugt

### Subtask 1.3 — Capsfilter als Plan B
Falls Ranking allein nicht reicht: in gstreamer_backend.rs:
- Nach uridecodebin's pad-added → audioconvert-Verkettung
- capsfilter setzen: audio/x-raw, rate=[44100,48000]
→ zwingt Decoder zur korrekten SBR-Samplerate

### Subtask 1.4 — Explizite Element-Insertion als Plan C
Falls Ranking + Capsfilter nicht ausreichen:
- In pad-added: prüfen ob Caps AAC sind (audio/mpeg, mpegversion=4)
- Wenn ja: aacparse → fdkaacdec als Bin zwischen uridecodebin und audioconvert
- Nachteil: Pipeline-Komplexität steigt
- Vorteil: volle Kontrolle über AAC-Decodierung

### Subtask 1.5 — Regression-Testing
- KEXP AAC+ (HE-AAC v1): http://live-aacplus-64.kexp.org/kexp64.aac
- SomaFM AAC (HE-AAC?): http://ice.somafm.com/groovesalad-aac-64
- MP3: http://live-mp3-128.kexp.org/kexp128.mp3
- Opus: beliebiger Opus-Stream
- OGG Vorbis: nettime-Streams
- A/B-Vergleich mit VLC für AAC+
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->

<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->

<!-- SECTION:FINAL_SUMMARY:END -->

## Definition of Done

<!-- DOD:BEGIN -->
- [ ] #1 Alle AC-Items checked
- [ ] #2 Manueller A/B-Test: KEXP AAC+ in Shortwave vs VLC — gleiche Qualität
- [ ] #3 Manueller Test: MP3-Stream läuft noch (KEXP MP3)
- [ ] #4 Manueller Test: SomaFM AAC-Stream klingt korrekt
- [ ] #5 cargo clippy --all -- -D warnings pass
<!-- DOD:END -->
