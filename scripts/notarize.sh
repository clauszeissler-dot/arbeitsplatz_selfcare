#!/bin/bash
# Run after a signed production build and local Keychain profile setup.
set -euo pipefail
cd "$(dirname "$0")/.."
PROFILE="${1:-lotse}"
IDENTITY='Developer ID Application: Claus Zeissler (ZWT6395G7Q)'
APP="$PWD/target/release/bundle/macos/Lotse.app"
OUT="$PWD/target/release/notarized"
mkdir -p "$OUT"
codesign --verify --deep --strict "$APP"
xcrun notarytool history --keychain-profile "$PROFILE" > /dev/null

submit() {
  local artifact="$1" label="$2" submission_id
  xcrun notarytool submit "$artifact" --keychain-profile "$PROFILE" --wait --output-format json > "$OUT/$label-result.json"
  submission_id=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["id"])' "$OUT/$label-result.json")
  xcrun notarytool log "$submission_id" --keychain-profile "$PROFILE" "$OUT/$label-log.json"
  python3 -c 'import json,sys; result=json.load(open(sys.argv[1])); print(result); sys.exit(0 if result.get("status")=="Accepted" else 1)' "$OUT/$label-result.json"
}

ditto -c -k --keepParent "$APP" "$OUT/Lotse.zip"
submit "$OUT/Lotse.zip" app
xcrun stapler staple "$APP"
xcrun stapler validate "$APP"
codesign --verify --deep --strict "$APP"
spctl --assess --type execute --verbose=2 "$APP"
# Recreate the archive so the delivered copy contains its offline ticket.
rm "$OUT/Lotse.zip"
ditto -c -k --keepParent "$APP" "$OUT/Lotse.zip"
STAGING=$(mktemp -d "${TMPDIR:-/tmp}/lotse-notary.XXXXXX")
trap 'rm -rf "$STAGING"' EXIT
ditto "$APP" "$STAGING/Lotse.app"
ln -s /Applications "$STAGING/Applications"
hdiutil create -volname Lotse -srcfolder "$STAGING" -ov -format UDZO "$OUT/Lotse.dmg"
codesign --force --sign "$IDENTITY" --timestamp "$OUT/Lotse.dmg"
submit "$OUT/Lotse.dmg" dmg
xcrun stapler staple "$OUT/Lotse.dmg"
xcrun stapler validate "$OUT/Lotse.dmg"
hdiutil verify "$OUT/Lotse.dmg"
printf 'Notarized app, ZIP and DMG verified: %s\n' "$OUT"
