---
id: doc-2
title: Dev-Build-Prozess und Script
status: published
created_date: '2026-05-24 14:00'
updated_date: '2026-05-24 14:00'
---

# Dev-Build-Prozess und Script

Shortwave nutzt **Meson + Ninja + Cargo** als Build-System. Für die tägliche Entwicklung ist der reine Meson-Weg zu langsam, weil er bei jedem Build alle Rust-Dependencies neu kompiliert. Dieses Doc erklärt den Build-Prozess, die Fallstricke und das Dev-Script `tmp/dev-build.sh`.

---

## 1. Build-Architektur (3 Schichten)

### Meson (Meta-Build)
`meson setup build` generiert die Build-Konfiguration (einmalig). Es:
- Setzt `MESON_*` Environment-Variablen (aus `src/meson.build`)
- Ruft `glib-compile-resources` auf um `.gresource` aus `.ui` XML-Templates zu machen (nötig für UI-Änderungen)
- Ruft `glib-compile-schemas` auf für die GSettings-Schemas
- Erzeugt Ninja-Targets für alles inkl. `cargo build --target-dir build/target`

### Ninja (Task-Executor)
`ninja -C build <target>` führt einzelne Build-Schritte aus. Der Cargo-Target-dir liegt in `build/target/` — das bewirkt dass bei jedem Meson-Durchlauf alle Rust-Dependencies NEU kompiliert werden (2-4 Minuten).

### Cargo (Rust-Compiler)
`cargo build --release` kompiliert die Rust-Quellen. Die `MESON_*` Envs werden zur **Compile-Zeit** via `option_env!()` Makro in `src/config.rs` in den Binary eingebacken.

---

## 2. Der `option_env!()` Fallstrick

```rust
// src/config.rs
macro_rules! config_var {
    ($name:ident) => {
        pub static $name: LazyLock<&'static str> = LazyLock::new(|| {
            option_env!(concat!("MESON_", stringify!($name)))
                .expect(concat!("MESON_", stringify!($name), " was not set at compile time"))
        });
    };
}
```

`option_env!("MESON_NAME")` wird beim **cargo compile** ausgewertet. Wenn die Variable dann nicht gesetzt ist -> `expect()` panict -> Binary startet nicht.

**Das Tückische**: Cargo cached den kompilierten Code anhand der Source-Fingerprints. Wenn du `cargo build` OHNE die Envs laufen lässt und danach MIT den Envs, denkt cargo "Quellen unverändert, nimm Cache". Du kriegst den alten (kaputten) Binary.

**Lösung**: `touch src/config.rs` vor dem Build forciert die Neukompilierung von config.rs und allen Abhängigkeiten.

---

## 3. `MESON_*` Env-Vars im Detail

| Variable | Wert für Dev | Zweck |
|----------|-------------|-------|
| `MESON_DATADIR` | `tmp/shortwave-dev/share` | Pfad wo der Binary zur Laufzeit die `.gresource` sucht |
| `MESON_PKGNAME` | `shortwave` | Subdir-Name: `DATADIR/shortwave/...` |
| `MESON_APP_ID` | `de.haeckerfelix.Shortwave` | App-ID für GResource-Dateinamen |
| `MESON_PATH_ID` | `/de/haeckerfelix/Shortwave` | Pfad-ID |
| `MESON_VERSION` | `5.1.0` | Versionsnummer |
| `MESON_PROFILE` | `development` | Schaltet Dev-Modus (`.Devel` App-ID Suffix) |
| `MESON_VCS_TAG` | `dev` | Git-Tag |
| `MESON_LOCALEDIR` | `/usr/share/locale` | Übersetzungs-Pfad |
| `MESON_NAME` | `Shortwave` | Anzeigename |

---

## 4. Warum nicht einfach Meson?

Weil Meson `cargo build --target-dir build/target` setzt. Der Target-dir ist jedes Mal LEER bei einem frischen Meson-Setup — alle Dependencies werden von Null kompiliert (2-4 Minuten).

Unser Ansatz:
- **Meson/Ninja** nur für GResource + GSchema (2 Sekunden)
- **Cargo direkt** mit default `target/` (30-60s, Dependencies sind gecached)

---

## 5. Dev-Script: `tmp/dev-build.sh`

Ein Skript das den gesamten Build managed inkl. State-Prüfung.

### Verwendung

```bash
./tmp/dev-build.sh            # Schema + GResource + Binary + Clippy + Tests
./tmp/dev-build.sh run        # Baut falls nötig + startet die App
./tmp/dev-build.sh check      # Nur Diagnose, kein Build
./tmp/dev-build.sh binary     # Nur Rust-Binary (forciert Rebuild via touch src/config.rs)
./tmp/dev-build.sh gresource  # Nur GResource aus Meson-Build kopieren
./tmp/dev-build.sh test       # Tests + Clippy
./tmp/dev-build.sh full       # Wie default + Diagnose am Ende
```

### Was das Script macht

1. **Diagnose** (`check`): Zeigt Branch, Binary-Größe/Alter, GResource-Status, Schema-Status, Testergebnisse
2. **GSettings Schema**: Kopiert aus Meson-Build und kompiliert mit `glib-compile-schemas`
3. **GResource**: Baut mit `ninja -C build data/...gresource` und kopiert nach `tmp/shortwave-dev/share/shortwave/`
4. **Binary**: Setzt alle `MESON_*` Envs, `touch src/config.rs` (Cache-Buster), `cargo build --release`
5. **Run**: Setzt `GSETTINGS_SCHEMA_DIR` und startet den Binary

### App manuell starten

```bash
GSETTINGS_SCHEMA_DIR=tmp/shortwave-dev/share/glib-2.0/schemas ./target/release/shortwave
```
