#!/bin/bash
# Run after a signed production build and local Keychain profile setup.
set -euo pipefail
cd "$(dirname "$0")/.."
# Explicit inputs prevent a fork from silently selecting the original developer.
if [[ $# -ne 1 || -z "${1:-}" || -z "${APPLE_SIGNING_IDENTITY:-}" || -z "${APPLE_EXPECTED_TEAM_ID:-}" || -z "${APPLE_EXPECTED_BUNDLE_ID:-}" ]]; then
  printf '%s\n' 'Usage: APPLE_SIGNING_IDENTITY="Developer ID Application: …" APPLE_EXPECTED_TEAM_ID="TEAMID" APPLE_EXPECTED_BUNDLE_ID="com.example.app" [APP_PATH="path/to/YourApp.app"] ./scripts/notarize.sh your-keychain-profile' >&2
  exit 2
fi
PROFILE="$1"
IDENTITY="$APPLE_SIGNING_IDENTITY"
EXPECTED_TEAM_ID="$APPLE_EXPECTED_TEAM_ID"
EXPECTED_BUNDLE_ID="$APPLE_EXPECTED_BUNDLE_ID"
APP="${APP_PATH:-$PWD/target/release/bundle/macos/Lotse.app}"
APP_NAME="$(basename "$APP" .app)"
OUT="$PWD/target/release/notarized"
codesign --verify --deep --strict "$APP"
SIGNATURE_METADATA="$(codesign --display --verbose=4 "$APP" 2>&1)"
ACTUAL_IDENTITY="$(printf '%s\n' "$SIGNATURE_METADATA" | sed -n 's/^Authority=//p' | head -n 1)"
ACTUAL_TEAM_ID="$(printf '%s\n' "$SIGNATURE_METADATA" | sed -n 's/^TeamIdentifier=//p' | head -n 1)"
ACTUAL_BUNDLE_ID="$(printf '%s\n' "$SIGNATURE_METADATA" | sed -n 's/^Identifier=//p' | head -n 1)"
if [[ "$ACTUAL_IDENTITY" != "$IDENTITY" ]]; then
  printf 'Unerwartete Signieridentität: erwartet %s, gefunden %s\n' "$IDENTITY" "${ACTUAL_IDENTITY:-<leer>}" >&2
  exit 3
fi
if [[ "$ACTUAL_TEAM_ID" != "$EXPECTED_TEAM_ID" ]]; then
  printf 'Unerwartete Team-ID: erwartet %s, gefunden %s\n' "$EXPECTED_TEAM_ID" "${ACTUAL_TEAM_ID:-<leer>}" >&2
  exit 3
fi
if [[ "$ACTUAL_BUNDLE_ID" != "$EXPECTED_BUNDLE_ID" ]]; then
  printf 'Unerwartete Bundle-ID: erwartet %s, gefunden %s\n' "$EXPECTED_BUNDLE_ID" "${ACTUAL_BUNDLE_ID:-<leer>}" >&2
  exit 3
fi
mkdir -p "$OUT"
xcrun notarytool history --keychain-profile "$PROFILE" > /dev/null

submit() {
  local artifact="$1" label="$2" submission_id
  xcrun notarytool submit "$artifact" --keychain-profile "$PROFILE" --wait --output-format json > "$OUT/$label-result.json"
  submission_id=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["id"])' "$OUT/$label-result.json")
  xcrun notarytool log "$submission_id" --keychain-profile "$PROFILE" "$OUT/$label-log.json"
  python3 -c 'import json,sys; result=json.load(open(sys.argv[1])); print(result); sys.exit(0 if result.get("status")=="Accepted" else 1)' "$OUT/$label-result.json"
}

ditto -c -k --keepParent "$APP" "$OUT/$APP_NAME.zip"
submit "$OUT/$APP_NAME.zip" app
xcrun stapler staple "$APP"
xcrun stapler validate "$APP"
codesign --verify --deep --strict "$APP"
spctl --assess --type execute --verbose=2 "$APP"
# Recreate the archive so the delivered copy contains its offline ticket.
rm "$OUT/$APP_NAME.zip"
ditto -c -k --keepParent "$APP" "$OUT/$APP_NAME.zip"
STAGING=$(mktemp -d "${TMPDIR:-/tmp}/lotse-notary.XXXXXX")
trap 'rm -rf "$STAGING"' EXIT
ditto "$APP" "$STAGING/$APP_NAME.app"
ln -s /Applications "$STAGING/Applications"
hdiutil create -volname "$APP_NAME" -srcfolder "$STAGING" -ov -format UDZO "$OUT/$APP_NAME.dmg"
codesign --force --sign "$IDENTITY" --timestamp "$OUT/$APP_NAME.dmg"
submit "$OUT/$APP_NAME.dmg" dmg
xcrun stapler staple "$OUT/$APP_NAME.dmg"
xcrun stapler validate "$OUT/$APP_NAME.dmg"
hdiutil verify "$OUT/$APP_NAME.dmg"
printf 'Notarized app, ZIP and DMG verified: %s\n' "$OUT"
