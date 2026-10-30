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

# Alle fünf Versionsstellen müssen mit dem Tag übereinstimmen
ACTUAL="$(python3 "$(dirname "$0")/versioning.py" check)"
if [[ "$ACTUAL" != "$VERSION" ]]; then
  printf 'Versionsabweichung: Tag %s, Quelle %s\n' "$VERSION" "$ACTUAL" >&2
  exit 3
fi
# Ohne Changelog-Abschnitt gibt es keinen Release
python3 "$(dirname "$0")/versioning.py" changelog "$VERSION" > /dev/null
# Nur eine höhere Version als jede veröffentlichte wird als Update angeboten
git fetch --tags --quiet origin
PUBLISHED="$(git tag --list | grep -vxF "$TAG" || true)"
# shellcheck disable=SC2086
python3 "$(dirname "$0")/versioning.py" newer "$VERSION" $PUBLISHED
