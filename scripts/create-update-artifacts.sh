#!/bin/bash
set -euo pipefail

if [[ $# -ne 3 || ! "$1" =~ ^v([0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
  printf '%s\n' 'Usage: scripts/create-update-artifacts.sh vX.Y.Z /path/to/Lotse.app /path/to/output' >&2
  exit 2
fi
if [[ -z "${TAURI_SIGNING_PRIVATE_KEY:-}" || -z "${TAURI_SIGNING_PRIVATE_KEY_PASSWORD:-}" ]]; then
  printf '%s\n' 'Updater-Signierschlüssel oder Passwort fehlt.' >&2
  exit 2
fi

TAG="$1"
VERSION="${BASH_REMATCH[1]}"
APP="$2"
OUT="$3"
APP_NAME="$(basename "$APP")"
ARCHIVE="$OUT/$APP_NAME.tar.gz"

if [[ -z "$OUT" || "$OUT" == "/" ]]; then
  printf 'Unsicheres Ausgabeziel abgelehnt: %s\n' "$OUT" >&2
  exit 3
fi
if [[ ! -d "$APP" || "$APP_NAME" != "Lotse.app" ]]; then
  printf 'Ungültiges Update-App-Bundle: %s\n' "$APP" >&2
  exit 3
fi
APP_VERSION="$(python3 - "$APP/Contents/Info.plist" <<'PY'
import plistlib
from pathlib import Path
import sys

with Path(sys.argv[1]).open("rb") as handle:
    print(plistlib.load(handle)["CFBundleShortVersionString"])
PY
)"
if [[ "$APP_VERSION" != "$VERSION" ]]; then
  printf 'Updateversion stimmt nicht: Tag %s, App %s\n' "$VERSION" "$APP_VERSION" >&2
  exit 3
fi

mkdir -p "$OUT"
rm -f "$ARCHIVE" "$ARCHIVE.sig" "$OUT/latest.json"
COPYFILE_DISABLE=1 /usr/bin/tar -czf "$ARCHIVE" -C "$(dirname "$APP")" "$APP_NAME"
npm run tauri -- signer sign --app-version "$VERSION" "$ARCHIVE"

python3 - "$VERSION" "$TAG" "$ARCHIVE.sig" "$OUT/latest.json" <<'PY'
import datetime
import json
from pathlib import Path
import sys

version, tag, signature_path, output_path = sys.argv[1:]
signature = Path(signature_path).read_text().strip()
if not signature:
    raise SystemExit("Leere Updater-Signatur")
manifest = {
    "version": version,
    "notes": "Signiertes und von Apple notarisiertes Lotse-Update.",
    "pub_date": datetime.datetime.now(datetime.UTC).isoformat().replace("+00:00", "Z"),
    "platforms": {
        "macos-universal": {
            "url": f"https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/download/{tag}/Lotse.app.tar.gz",
            "signature": signature,
        }
    },
}
Path(output_path).write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
PY

printf 'Signierte Universal-Updateartefakte erstellt: %s\n' "$OUT"
