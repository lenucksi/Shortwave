---
id: doc-5
title: Discovery-Script-Engine-Architecture
type: other
created_date: 2026-05-26 20:40
updated_date: 2026-05-26 20:43
---
---
id: doc-5
title: "Discovery Script Engine — Rhai Architecture & Script Contract"
---

## Purpose

Define the boundary between the Rust host engine and Rhai scripts in Shortwave's Discovery Script Engine. Explains what lives where, why, and how users develop scripts.

## Core Principle

Rhai is **fully sandboxed** — no network, no filesystem, no process spawning, no system calls. Every external capability must be explicitly registered from the Rust host. This is not a limitation; it is the security model that makes Flathub distribution viable.

## Capability Split

### Rust Engine (`src/discovery/engine.rs`) — provides:

| Function | Signature | Purpose |
|---|---|---|
| `http_get(url)` | `str → String` | Fetch raw page/API response |
| `http_get_json(url)` | `str → Dynamic` | Fetch + JSON-parse in one call |
| `html_select(html, css)` | `(str, str) → String` | CSS selector → JSON array of matched nodes |
| `json_parse(string)` | `str → Dynamic` | Parse JSON string into Rhai Dynamic |
| `json_stringify(value)` | `Dynamic → str` | Serialize Rhai value to pretty JSON |

**Why these must be in Rust:**
- `http_get`/`http_get_json` — Rhai has no networking capability
- `html_select` — CSS selector parsing requires the `scraper` crate (Rust-only)
- `json_parse`/`json_stringify` — serde_json integration; Rhai's built-in JSON support is limited
- All functions return plain Rhai types (String, Dynamic) — no opaque custom types, no lifetime management

### Rhai Script — does:

| Operation | How |
|---|---|
| String manipulation | Rhai built-in: `split()`, `contains()`, `trim()`, `+` concatenation |
| Map/array construction | Rhai `#{ }` and `[ ]` literals |
| Loop over extracted nodes | `for node in nodes { }` |
| Access HTML attributes | `node.attrs["href"]` (after `json_parse(html_select(...))`) |
| Build station list | Push to array, construct map with string keys |
| Return result | `json_stringify(#{"provider": "...", "stations": [...]})` |

## Script Development Workflow

### 1. Reverse-engineer the target

Use browser DevTools (F12) to inspect the page. Find CSS selectors that uniquely identify the data: channel links, names, icons, stream URL patterns.

### 2. Write the Rhai script

```rust
let html = http_get("https://example.com/");
let nodes_json = html_select(html, "a.channel-link");
let nodes = json_parse(nodes_json);
// ... build station list ...
json_stringify(result)
```

### 3. Test with the runner

```bash
shortwave-script run myscript.rhai          # raw JSON output
shortwave-script run myscript.rhai --pretty  # human-readable station list
```

The runner executes the script against the real website. Use `--pretty` for a human-friendly station listing instead of raw JSON.

### 4. Deploy to Shortwave

Copy the `.rhai` file to either:
- **Bundled**: `/app/share/shortwave/providers/` (read-only in Flatpak, ships with app)
- **User**: `~/.var/app/de.haeckerfelix.Shortwave/data/shortwave/providers/` (writable, survives updates)

No rebuild needed. The app picks up scripts on next start.

## Why No Custom Rhai Types

SW-7 originally proposed `HtmlDoc` and `NodeRef` as Rhai custom types. This was rejected in favor of the simpler JSON-returning `html_select()` because:

1. **No lifetime management** — custom Rhai types with references to parser internals are fragile
2. **Scripts work with plain Dynamic** — `json_parse()` returns standard Rhai maps/arrays
3. **Same ergonomics** — `node.attrs["href"]` vs `html_attr(node, "href")` is negligible difference
4. **Less Rust code** — no custom type registration, no type-checking boilerplate
5. **Proven in prototype** — `tools/rhai-runner/` already uses this pattern, somafm.rhai works

## Script Contract

Every Discovery Script must return a JSON string matching this schema:

```json
{
  "provider": "Provider Name",
  "stations": [
    {
      "name": "Station Name",
      "stream_url": "https://...",
      "stream_urls": [
        { "url": "https://...", "codec": "MP3", "bitrate": 256, "tls": true }
      ],
      "homepage": "https://...",
      "icon_url": "https://...",
      "tags": "genre,country",
      "country": "US",
      "language": "en"
    }
  ]
}
```

Only `provider`, `name`, and `stream_url` are required. All other fields are optional (`stream_urls` defaults to empty).

## Sandbox Limits

| Setting | Value | Purpose |
|---|---|---|
| `max_operations` | 500,000 | Prevent infinite loops |
| `max_call_levels` | 50 | Prevent stack overflow via recursion |
| `max_string_size` | 10 MB | Prevent memory exhaustion |
| HTTP timeout | 10s | Prevent hanging on slow servers |

## Stretch: Provider-Aware Station Memory

Each station imported via a Discovery Script could store a `provider_id` + `script_hash` to enable:

- **Update checks**: re-run the originating script, match stations by slug/name, update URLs
- **Change detection**: flag stations whose data changed on the source website
- **Refresh button**: trigger re-import from a specific provider

This is future work (post-milestone).
