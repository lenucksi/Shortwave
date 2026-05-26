# Context Brief: Playlist-Support (PLS + M3U)

Classification: **L2** (cross-module: new `src/playlist/`, touches api, audio, ui)

## Closest Analogs

| What | Pattern | File |
|------|---------|------|
| Error types | `thiserror` enum with Display | `src/api/error.rs` |
| Public API re-exports | `pub use` from `mod.rs` | `src/api/mod.rs`, `src/audio/mod.rs` |
| HTTP requests | `crate::api::http::get(url)` | `src/api/http.rs` |
| GObject properties | `#[property(get, set)]` in `imp` struct | `src/api/station.rs` |
| Serialization | `#[derive(Serialize, Deserialize)]` + custom serde fns | `src/api/station_metadata.rs` |

## Naming Conventions

- **Files**: snake_case (`station_metadata.rs`, `gstreamer_backend.rs`)
- **Types**: CamelCase (`StationMetadata`, `GstreamerChange`, `SwPlayer`)
- **Functions**: snake_case (`parse_bus_message`, `set_source_uri`)
- **Error types**: PascalCase (`PlsError`, `PlaylistError`) — follow `api::Error`
- **Module pattern**: `src/playlist/mod.rs` exports, `src/playlist/pls.rs` internal

## Patterns to Follow

1. **Module structure**: new `src/playlist/` directory with `mod.rs`, `pls.rs`, `m3u.rs`, `fetch.rs`
2. **Public API**: `src/playlist/mod.rs` re-exports only `PlaylistEntry`, `fetch_and_parse`
3. **Error handling**: `#[derive(thiserror::Error)]` for `PlaylistError`
4. **HTTP**: reuse `crate::api::http::get(url)` via `async fn`
5. **Serialization**: `#[serde(default)]` on new `StationMetadata` fields for backward compat
6. **No comments** unless non-obvious (project convention)
7. **i18n**: wrap new user-visible strings with `i18n!()` / `i18n_f!()`

## Risks

1. **GStreamer pipeline** is fragile with dead URLs (no timeout). Fallback handles this at app level.
2. **StationMetadata** JSON in DB: existing stations lack `alternate_urls`. `#[serde(default)]` handles this.
3. **Add dialog** currently synchronous `update_metadata()`. Async PLS fetch needs `glib::spawn_future_local`.
4. **Cast sender**: fallback only for local playback, not Chromecast.

## Files to Touch

- `src/playlist/` (new) — parser + fetch
- `src/api/station_metadata.rs` — add fields
- `src/api/station.rs` — add `stream_urls()`
- `src/audio/player.rs` — add fallback logic
- `src/ui/add_station_dialog.rs` — PLS detection + async fetch
- `data/gtk/add_station_dialog.ui` — status label

## Ancillary Info

- `api::http::get(url)` returns `reqwest::Response` via dedicated HTTP thread
- `StationMetadata` is `#[derive(glib::Boxed, Default, Debug, Clone, Serialize, Deserialize)]`
- `SwPlayer` uses `async_channel` for GStreamer messages
- Pipeline: `uridecodebin ! audioconvert ! tee ! queue ! pulsesink`
