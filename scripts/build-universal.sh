#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT_ROOT="${CARGO_TARGET_DIR:-$ROOT/target}"
ARM_ROOT="$OUTPUT_ROOT/lotse-arm64"
X86_ROOT="$OUTPUT_ROOT/lotse-x86_64"
ARM_APP="$ARM_ROOT/aarch64-apple-darwin/release/bundle/macos/Lotse.app"
X86_APP="$X86_ROOT/x86_64-apple-darwin/release/bundle/macos/Lotse.app"
OUTPUT_APP="$OUTPUT_ROOT/universal-apple-darwin/release/bundle/macos/Lotse.app"

if [[ -z "$OUTPUT_ROOT" || "$OUTPUT_ROOT" == "/" ]]; then
  printf '%s\n' 'Unsicheres Build-Ziel abgewiesen.' >&2
  exit 2
fi

cd "$ROOT"
npm run build

CARGO_TARGET_DIR="$ARM_ROOT" cargo build -p lotse --target aarch64-apple-darwin --release --locked
CARGO_TARGET_DIR="$ARM_ROOT" npx tauri bundle --target aarch64-apple-darwin --bundles app --no-sign --ci

CARGO_TARGET_DIR="$X86_ROOT" cargo build -p lotse --target x86_64-apple-darwin --release --locked
CARGO_TARGET_DIR="$X86_ROOT" npx tauri bundle --target x86_64-apple-darwin --bundles app --no-sign --ci

[[ -d "$ARM_APP" && -d "$X86_APP" ]] || {
  printf '%s\n' 'Einer der architekturspezifischen App-Builds fehlt.' >&2
  exit 3
}
rm -rf "$OUTPUT_APP"
mkdir -p "$(dirname "$OUTPUT_APP")"
ditto "$ARM_APP" "$OUTPUT_APP"
lipo -create \
  "$ARM_APP/Contents/MacOS/Lotse" \
  "$X86_APP/Contents/MacOS/Lotse" \
  -output "$OUTPUT_APP/Contents/MacOS/Lotse"

ARCHS="$(lipo -archs "$OUTPUT_APP/Contents/MacOS/Lotse")"
[[ "$ARCHS" == *arm64* && "$ARCHS" == *x86_64* ]] || {
  printf 'Universal-Binary unvollständig: %s\n' "$ARCHS" >&2
  exit 3
}
printf 'Universal-App erstellt: %s (%s)\n' "$OUTPUT_APP" "$ARCHS"
