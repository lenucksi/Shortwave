#!/usr/bin/env bash
set -euo pipefail

# ──────────────────────────────────────────────
# Shortwave Dev Build Script
# ──────────────────────────────────────────────
# Baut gresource + Rust Binary mit den richtigen
# MESON_* Env Vars, ohne das ganze meson/ninja/cargo
# dependency hell neu durchlaufen zu müssen.
#
# Usage:
#   ./tmp/dev-build.sh            # build alles
#   ./tmp/dev-build.sh run        # build + start
#   ./tmp/dev-build.sh gresource  # nur gresource neu
#   ./tmp/dev-build.sh binary     # nur Rust binary neu
#   ./tmp/dev-build.sh check      # nur state check + diagnose
# ──────────────────────────────────────────────

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MESON_BUILD_DIR="$REPO_ROOT/build"
DATA_DIR="$REPO_ROOT/tmp/shortwave-dev"
GRESOURCE="$MESON_BUILD_DIR/data/de.haeckerfelix.Shortwave.gresource"
GRESOURCE_DEST="$DATA_DIR/share/shortwave/de.haeckerfelix.Shortwave.gresource"
SCHEMA_SRC="$MESON_BUILD_DIR/data/de.haeckerfelix.Shortwave.gschema.xml"
SCHEMA_DIR="$DATA_DIR/share/glib-2.0/schemas"
BINARY="$REPO_ROOT/target/release/shortwave"

# MESON_* müssen beim Rust-COMPILE gesetzt sein (option_env! Makro)
export MESON_DATADIR="$DATA_DIR/share"
export MESON_PKGNAME="shortwave"
export MESON_APP_ID="de.haeckerfelix.Shortwave"
export MESON_PATH_ID="/de/haeckerfelix/Shortwave"
export MESON_VERSION="5.1.0"
export MESON_PROFILE="development"
export MESON_VCS_TAG="dev"
export MESON_LOCALEDIR="/usr/share/locale"
export MESON_NAME="Shortwave"

# ── Diagnose ──────────────────────────────────
diagnose() {
    echo "─── Dev Build Diagnose ───"

    # Branch
    local branch
    branch=$(cd "$REPO_ROOT" && git branch --show-current 2>/dev/null || echo "N/A")
    echo "Branch:           $branch"

    # Binary existiert?
    if [ -f "$BINARY" ]; then
        local size
        size=$(stat -c%s "$BINARY" 2>/dev/null || stat -f%z "$BINARY" 2>/dev/null)
        local mtime
        mtime=$(stat -c%Y "$BINARY" 2>/dev/null || stat -f%m "$BINARY" 2>/dev/null)
        local now
        now=$(date +%s)
        local age=$(( (now - mtime) / 60 ))
        echo "Binary:           ${size} Bytes, vor ${age}min gebaut"
    else
        echo "Binary:           ✗ NICHT VORHANDEN"
    fi

    # GResource
    if [ -f "$GRESOURCE_DEST" ]; then
        echo "GResource:        ✓ $GRESOURCE_DEST"
    else
        echo "GResource:        ✗ NICHT VORHANDEN"
    fi

    # GResource Up-to-Date?
    if [ -f "$GRESOURCE_DEST" ] && [ -f "$GRESOURCE" ]; then
        local gr_src
        local gr_dst
        gr_src=$(stat -c%Y "$GRESOURCE" 2>/dev/null || stat -f%m "$GRESOURCE" 2>/dev/null)
        gr_dst=$(stat -c%Y "$GRESOURCE_DEST" 2>/dev/null || stat -f%m "$GRESOURCE_DEST" 2>/dev/null)
        if [ "$gr_dst" -ge "$gr_src" ]; then
            echo "GResource Sync:   ✓ aktuell"
        else
            echo "GResource Sync:   ✗ muss kopiert werden (Quelle neuer)"
        fi
    fi

    # GSettings Schema
    if [ -f "$SCHEMA_DIR/gschemas.compiled" ]; then
        echo "GSettings Schema: ✓ kompiliert"
    else
        echo "GSettings Schema: ✗ fehlt (glib-compile-schemas nötig)"
    fi

    # Meson Build Dir
    if [ -d "$MESON_BUILD_DIR" ]; then
        echo "Meson Build:      ✓ $MESON_BUILD_DIR"
    else
        echo "Meson Build:      ✗ fehlt ('meson setup build' nötig)"
    fi

    # Letzte Tests
    local tests_cache="$REPO_ROOT/target/release/.test-summary"
    if [ -f "$tests_cache" ]; then
        echo "Tests:            $(cat "$tests_cache")"
    else
        echo "Tests:            ? (noch nicht gelaufen)"
    fi

    echo "────────────────────────────"
}

# ── GResource bauen ────────────────────────────
build_gresource() {
    echo "─── GResource bauen ───"
    if [ ! -d "$MESON_BUILD_DIR" ]; then
        echo "→ Meson build dir existiert nicht. Setup läuft..."
        cd "$REPO_ROOT" && meson setup --prefix="$DATA_DIR" "$MESON_BUILD_DIR" >/dev/null 2>&1
    fi
    cd "$REPO_ROOT" && ninja -C "$MESON_BUILD_DIR" data/de.haeckerfelix.Shortwave.gresource 2>&1 | tail -1
    mkdir -p "$(dirname "$GRESOURCE_DEST")"
    cp "$GRESOURCE" "$GRESOURCE_DEST"
    echo "→ GResource kopiert nach $GRESOURCE_DEST"
}

# ── GSettings Schema ───────────────────────────
build_schema() {
    echo "─── GSettings Schema ───"
    if [ ! -f "$SCHEMA_SRC" ]; then
        # Schema existiert im build dir, wurde vom meson setup generiert
        if [ ! -d "$MESON_BUILD_DIR" ]; then
            echo "→ Meson build dir fehlt. Setup läuft..."
            cd "$REPO_ROOT" && meson setup --prefix="$DATA_DIR" "$MESON_BUILD_DIR" >/dev/null 2>&1
        fi
    fi
    mkdir -p "$SCHEMA_DIR"
    if [ -f "$SCHEMA_SRC" ]; then
        cp "$SCHEMA_SRC" "$SCHEMA_DIR/"
    fi
    glib-compile-schemas "$SCHEMA_DIR" 2>&1
    echo "→ Schema kompiliert: $SCHEMA_DIR/gschemas.compiled"
}

# ── Rust Binary bauen ──────────────────────────
build_binary() {
    echo "─── Rust Binary bauen ───"
    # Force rebuild: config.rs cached sonst falsch wegen option_env!
    touch "$REPO_ROOT/src/config.rs"
    cd "$REPO_ROOT" && cargo build --release 2>&1 | tail -3
    echo "→ Binary: $BINARY"
}

# ── Tests ──────────────────────────────────────
run_tests() {
    echo "─── Tests ───"
    cd "$REPO_ROOT" && cargo test -p shortwave 2>&1 | tail -3
    local result
    result=$(cd "$REPO_ROOT" && cargo test -p shortwave 2>&1 | grep "^test result:" | head -1)
    mkdir -p "$(dirname "$REPO_ROOT/target/release/.test-summary")"
    echo "$result" > "$REPO_ROOT/target/release/.test-summary"
    echo "$result"
}

# ── Clippy ─────────────────────────────────────
run_clippy() {
    echo "─── Clippy ───"
    cd "$REPO_ROOT" && cargo clippy --all -- -D warnings 2>&1 | tail -3
}

# ── Run ────────────────────────────────────────
run_app() {
    echo "─── App starten ───"
    local gsettings_dir="$DATA_DIR/share/glib-2.0/schemas"
    echo "GSETTINGS_SCHEMA_DIR=$gsettings_dir $BINARY"
    GSETTINGS_SCHEMA_DIR="$gsettings_dir" "$BINARY"
}

# ── Main ───────────────────────────────────────
cmd="${1:-all}"

case "$cmd" in
    all)
        build_schema
        build_gresource
        build_binary
        run_clippy
        run_tests
        echo "✔ Fertig. Starte mit:  GSETTINGS_SCHEMA_DIR=$DATA_DIR/share/glib-2.0/schemas $BINARY"
        ;;
    run)
        # Prüfe ob binary aktuell ist, bau wenn nötig
        if [ ! -f "$BINARY" ]; then
            build_schema
            build_gresource
            build_binary
        fi
        run_app
        ;;
    gresource)
        build_gresource
        ;;
    binary)
        build_binary
        ;;
    schema)
        build_schema
        ;;
    test)
        run_tests
        ;;
    clippy)
        run_clippy
        ;;
    check|diagnose)
        diagnose
        ;;
    full)
        build_schema
        build_gresource
        build_binary
        run_clippy
        run_tests
        echo ""
        diagnose
        echo ""
        echo "✔ Alles gebaut. Starte mit:"
        echo "  GSETTINGS_SCHEMA_DIR=$DATA_DIR/share/glib-2.0/schemas $BINARY"
        ;;
    *)
        echo "Usage: $0 {all|run|gresource|binary|schema|test|clippy|check|full}"
        echo ""
        echo "  all       = schema + gresource + binary + clippy + tests (default)"
        echo "  run       = build falls nötig + app starten"
        echo "  gresource = nur gresource aus meson build kopieren"
        echo "  binary    = nur Rust binary bauen"
        echo "  schema    = GSettings Schema kompilieren"
        echo "  test      = Tests laufen lassen"
        echo "  clippy    = Clippy check"
        echo "  check     = State diagnose ohne Build"
        echo "  full      = alles inkl. Diagnose"
        exit 1
        ;;
esac
