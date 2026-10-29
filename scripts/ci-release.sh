#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 || ! "$1" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  printf '%s\n' 'Usage: scripts/ci-release.sh vX.Y.Z' >&2
  exit 2
fi

required=(
  GH_TOKEN APPLE_CERTIFICATE APPLE_CERTIFICATE_PASSWORD APPLE_SIGNING_IDENTITY
  APPLE_EXPECTED_TEAM_ID APPLE_EXPECTED_BUNDLE_ID KEYCHAIN_PASSWORD
  APPLE_API_ISSUER APPLE_API_KEY APPLE_API_KEY_CONTENT
  TAURI_SIGNING_PRIVATE_KEY TAURI_SIGNING_PRIVATE_KEY_PASSWORD
)
for name in "${required[@]}"; do
  if [[ -z "${!name:-}" ]]; then
    printf 'Fehlende Release-Konfiguration: %s\n' "$name" >&2
    exit 2
  fi
done

TAG="$1"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
INPUT="$ROOT/target/release-input"
WORK="${RUNNER_TEMP:-$(mktemp -d)}/lotse-release"
KEYCHAIN="$WORK/lotse-signing.keychain-db"
CERTIFICATE="$WORK/developer-id.p12"
API_KEY_PATH="$WORK/AuthKey.p8"
APP="$WORK/Lotse.app"
PROFILE="lotse-ci-${GITHUB_RUN_ID:-local}"

cleanup() {
  security delete-keychain "$KEYCHAIN" >/dev/null 2>&1 || true
  rm -rf "$WORK"
}
trap cleanup EXIT
mkdir -p "$WORK"

cd "$INPUT"
shasum -a 256 -c SHA256SUMS
ditto -x -k Lotse-unsigned.zip "$WORK"
[[ -d "$APP" ]] || {
  printf '%s\n' 'Das geprüfte Release-Artefakt enthält keine Lotse.app.' >&2
  exit 3
}

printf '%s' "$APPLE_CERTIFICATE" | base64 -D > "$CERTIFICATE"
printf '%s' "$APPLE_API_KEY_CONTENT" > "$API_KEY_PATH"
chmod 600 "$CERTIFICATE" "$API_KEY_PATH"
security create-keychain -p "$KEYCHAIN_PASSWORD" "$KEYCHAIN"
security set-keychain-settings -lut 21600 "$KEYCHAIN"
security unlock-keychain -p "$KEYCHAIN_PASSWORD" "$KEYCHAIN"
security import "$CERTIFICATE" -k "$KEYCHAIN" -P "$APPLE_CERTIFICATE_PASSWORD" -T /usr/bin/codesign
security set-key-partition-list -S apple-tool:,apple: -s -k "$KEYCHAIN_PASSWORD" "$KEYCHAIN"
security list-keychains -d user -s "$KEYCHAIN"

codesign --force --deep --strict --options runtime --timestamp \
  --entitlements "$ROOT/src-tauri/Entitlements.plist" \
  --sign "$APPLE_SIGNING_IDENTITY" "$APP"
xcrun notarytool store-credentials "$PROFILE" \
  --key "$API_KEY_PATH" --key-id "$APPLE_API_KEY" --issuer "$APPLE_API_ISSUER"

cd "$ROOT"
APP_PATH="$APP" scripts/notarize.sh "$PROFILE"
scripts/create-update-artifacts.sh "$TAG" "$APP" "$ROOT/target/release/notarized"
cd target/release/notarized
shasum -a 256 Lotse.dmg Lotse.zip Lotse.app.tar.gz Lotse.app.tar.gz.sig latest.json > SHA256SUMS
gh release create "$TAG" Lotse.dmg Lotse.zip Lotse.app.tar.gz Lotse.app.tar.gz.sig latest.json SHA256SUMS \
  "$ROOT/LICENSE" "$ROOT/NOTICE" "$ROOT/BRANDING.md" \
  --draft \
  --title "Lotse $TAG" \
  --notes "Signiert und von Apple notarisiert. DMG öffnen, Lotse in Programme ziehen."
