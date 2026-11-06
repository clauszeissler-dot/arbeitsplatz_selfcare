#!/bin/bash
# Lokaler Build der Host-Architektur: signieren, nach /Applications installieren, starten.
# Benötigt APPLE_SIGNING_IDENTITY (eigene Developer ID). Die App zeigt in den Einstellungen
# „lokaler Build (<Git-Hash>)“ und lässt sich so nie mit einem Release verwechseln.
# Die Autostart-Datei wird nur gelesen und gemeldet, nie geändert.
set -euo pipefail

if [[ -z "${APPLE_SIGNING_IDENTITY:-}" ]]; then
  printf '%s\n' 'Usage: APPLE_SIGNING_IDENTITY="<deine Developer-ID-Identität>" scripts/build-local.sh' >&2
  exit 2
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
VERSION="$(python3 scripts/versioning.py check)"
DIRTY=""
[[ -z "$(git status --porcelain -- . ':!docs/handoffs')" ]] || DIRTY="-dirty"
export LOTSE_BUILD="local-$(git rev-parse --short HEAD)${DIRTY}"

APP="$ROOT/target/release/bundle/macos/Lotse.app"
INSTALLED="/Applications/Lotse.app"
BACKUP="$ROOT/target/local-backup/Lotse.app"

npm run tauri -- build --no-sign --bundles app
codesign --force --deep --options runtime --timestamp \
  --entitlements "$ROOT/src-tauri/Entitlements.plist" \
  --sign "$APPLE_SIGNING_IDENTITY" "$APP"
codesign --verify --deep --strict "$APP"

pkill -x lotse || true
sleep 2
if [[ -d "$INSTALLED" ]]; then
  rm -rf "$BACKUP"
  mkdir -p "$(dirname "$BACKUP")"
  ditto "$INSTALLED" "$BACKUP"
fi
rm -rf "$INSTALLED"
ditto "$APP" "$INSTALLED"
open "$INSTALLED"

printf 'Installiert: Lotse %s (%s) in %s\n' "$VERSION" "$LOTSE_BUILD" "$INSTALLED"
AUTOSTART="$(plutil -extract ProgramArguments.0 raw "$HOME/Library/LaunchAgents/Lotse.plist" 2>/dev/null || true)"
if [[ -n "$AUTOSTART" && "$AUTOSTART" != "$INSTALLED/Contents/MacOS/lotse" ]]; then
  printf 'Achtung: Autostart zeigt auf %s. In den Lotse-Einstellungen „Beim Anmelden starten“ aus- und wieder einschalten.\n' "$AUTOSTART" >&2
fi
