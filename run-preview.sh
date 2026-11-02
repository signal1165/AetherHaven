#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

BIN="$ROOT/target/debug/aetherhaven_launcher"

if [[ ! -x "$BIN" ]]; then
  echo "Preview binary not found; building it once..."
  cargo build -p aetherhaven_launcher
fi

export AETHERHAVEN_PREVIEW=1
exec "$BIN" "$@"
