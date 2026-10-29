#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 || ! "$1" =~ ^v([0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
  printf '%s\n' 'Release-Tag muss dem Format vX.Y.Z entsprechen.' >&2
  exit 2
fi

TAG="$1"
VERSION="${BASH_REMATCH[1]}"
git show-ref --verify --quiet "refs/tags/$TAG" || {
  printf 'Release-Tag existiert nicht: %s\n' "$TAG" >&2
  exit 3
}
TAG_COMMIT="$(git rev-list -n 1 "$TAG")"
HEAD_COMMIT="$(git rev-parse HEAD)"
if [[ "$TAG_COMMIT" != "$HEAD_COMMIT" ]]; then
  printf 'Checkout %s stimmt nicht mit %s (%s) überein.\n' "$HEAD_COMMIT" "$TAG" "$TAG_COMMIT" >&2
  exit 3
fi
git fetch --no-tags origin main
git merge-base --is-ancestor "$TAG_COMMIT" origin/main || {
  printf '%s\n' 'Release-Commit liegt nicht auf origin/main.' >&2
  exit 3
}

python3 - "$VERSION" <<'PY'
import json
import pathlib
import re
import sys

expected = sys.argv[1]
root = pathlib.Path.cwd()
package = json.loads((root / "package.json").read_text())["version"]
tauri = json.loads((root / "src-tauri" / "tauri.conf.json").read_text())["version"]
cargo_text = (root / "Cargo.toml").read_text()
cargo_match = re.search(r'^version\s*=\s*"([^"]+)"', cargo_text, re.MULTILINE)
cargo = cargo_match.group(1) if cargo_match else None
actual = {"Cargo.toml": cargo, "package.json": package, "tauri.conf.json": tauri}
wrong = {name: value for name, value in actual.items() if value != expected}
if wrong:
    raise SystemExit(f"Versionsabweichung zu {expected}: {wrong}")
PY
