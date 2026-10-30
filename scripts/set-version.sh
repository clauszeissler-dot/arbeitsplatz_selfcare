#!/bin/bash
# Setzt die Lotse-Version an allen fünf Stellen (Cargo.toml, Cargo.lock, package.json,
# package-lock.json, tauri.conf.json). Versionen nie von Hand ändern.
set -euo pipefail

if [[ $# -ne 1 ]]; then
  printf '%s\n' 'Usage: scripts/set-version.sh X.Y.Z' >&2
  exit 2
fi
exec python3 "$(dirname "$0")/versioning.py" set "$1"
