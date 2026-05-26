#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DATA_DIR="$REPO_ROOT/tmp/shortwave-dev"
BINARY="$REPO_ROOT/target/release/shortwave"
GSETTINGS_SCHEMA_DIR="$DATA_DIR/share/glib-2.0/schemas" "$BINARY"
