#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

# Setup mode must never bypass onboarding.
unset AETHERHAVEN_PREVIEW || true

# Keep this test installation isolated from any existing AetherHaven data.
export XDG_DATA_HOME="${AETHERHAVEN_SETUP_DATA_HOME:-$ROOT/.aetherhaven-setup-data}"
mkdir -p "$XDG_DATA_HOME"

BIN="$ROOT/target/debug/aetherhaven_launcher"
if [[ ! -x "$BIN" ]]; then
  echo "Building AetherHaven Launcher..."
  cargo build -p aetherhaven_launcher
fi

exec "$BIN" "$@"
