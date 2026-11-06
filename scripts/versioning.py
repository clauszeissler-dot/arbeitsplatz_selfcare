#!/usr/bin/env python3
"""Versionierung für Lotse (A-030): eine Quelle, ein Bump-Befehl, Changelog und Release-Reihenfolge.

Quelle ist `[workspace.package] version` in Cargo.toml. Dieselbe Version steht in
tauri.conf.json, package.json, package-lock.json und Cargo.lock und wird nur
über dieses Werkzeug geändert.

  versioning.py [--root DIR] check              gibt die Version aus, Exit 1 bei Abweichung
  versioning.py [--root DIR] set X.Y.Z          schreibt alle fünf Stellen
  versioning.py [--root DIR] changelog X.Y.Z    gibt den Changelog-Abschnitt aus (Exit 1 ohne Inhalt)
  versioning.py [--root DIR] newer X.Y.Z [TAG…] Exit 1, wenn ein veröffentlichter Tag nicht älter ist
"""
import argparse
import json
from pathlib import Path
import re
import sys

SEMVER = re.compile(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$")
TAG = re.compile(r"^(?:v|lotse-)(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$")
CARGO_TOML = re.compile(r'(?m)(^\[workspace\.package\]\n(?:(?!^\[).*\n)*?version\s*=\s*")([^"]+)(")')
CARGO_LOCK = re.compile(r'(name = "(?:lotse|lotse-core)"\nversion = ")([^"]+)(")')
JSON_TOP_LEVEL = re.compile(r'(?m)^(  "version": ")([^"]+)(")')
PACKAGE_LOCK_ROOT = re.compile(r'(?m)(^  "packages": \{\n    "": \{\n(?:(?!^    \}).*\n)*?      "version": ")([^"]+)(")')


def read_versions(root: Path) -> dict[str, list[str]]:
    versions: dict[str, list[str]] = {}
    cargo = CARGO_TOML.search((root / "Cargo.toml").read_text())
    versions["Cargo.toml"] = [cargo.group(2)] if cargo else []
    versions["Cargo.lock"] = [m.group(2) for m in CARGO_LOCK.finditer((root / "Cargo.lock").read_text())]
    versions["package.json"] = [json.loads((root / "package.json").read_text())["version"]]
    lock = json.loads((root / "package-lock.json").read_text())
    versions["package-lock.json"] = [lock["version"], lock["packages"][""]["version"]]
    versions["src-tauri/tauri.conf.json"] = [
        json.loads((root / "src-tauri" / "tauri.conf.json").read_text())["version"]
    ]
    return versions


def check(root: Path) -> int:
    versions = read_versions(root)
    expected = (versions["Cargo.toml"] or [None])[0]
    wrong = {
        name: found
        for name, found in versions.items()
        if not found or any(value != expected for value in found)
    }
    if expected is None or not SEMVER.match(expected) or wrong:
        for name, found in wrong.items():
            print(f"Versionsabweichung in {name}: {found or 'nicht gefunden'} (erwartet {expected})", file=sys.stderr)
        if expected is None or not SEMVER.match(expected or ""):
            print(f"Quelle Cargo.toml hat keine gültige Version: {expected!r}", file=sys.stderr)
        return 1
    print(expected)
    return 0


def replace_once(text: str, pattern: re.Pattern, version: str, name: str, count: int = 1) -> str:
    new, n = pattern.subn(lambda m: m.group(1) + version + m.group(3), text, count=count)
    if n == 0:
        raise SystemExit(f"Versionsstelle in {name} nicht gefunden")
    return new


def set_version(root: Path, version: str) -> int:
    if not SEMVER.match(version):
        print(f"Ungültige Version {version!r}: erlaubt ist nur X.Y.Z ohne Suffix.", file=sys.stderr)
        return 2
    edits = {
        "Cargo.toml": lambda t: replace_once(t, CARGO_TOML, version, "Cargo.toml"),
        "Cargo.lock": lambda t: replace_once(t, CARGO_LOCK, version, "Cargo.lock", count=0),
        "package.json": lambda t: replace_once(t, JSON_TOP_LEVEL, version, "package.json"),
        "package-lock.json": lambda t: replace_once(
            replace_once(t, JSON_TOP_LEVEL, version, "package-lock.json"),
            PACKAGE_LOCK_ROOT,
            version,
            "package-lock.json (packages)",
        ),
        "src-tauri/tauri.conf.json": lambda t: replace_once(t, JSON_TOP_LEVEL, version, "tauri.conf.json"),
    }
    # Erst alles berechnen, dann schreiben: ein Fehler lässt die Dateien unverändert.
    new_texts = {name: edit((root / name).read_text()) for name, edit in edits.items()}
    for name, text in new_texts.items():
        (root / name).write_text(text)
    return check(root)


def changelog_section(root: Path, version: str) -> str | None:
    path = root / "CHANGELOG.md"
    if not path.exists():
        return None
    lines = path.read_text().splitlines()
    heading = re.compile(rf"^## \[{re.escape(version)}\]")
    start = next((i for i, line in enumerate(lines) if heading.match(line)), None)
    if start is None:
        return None
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("## [")), len(lines))
    body = "\n".join(lines[start + 1 : end]).strip()
    return body or None


def newer(version: str, tags: list[str]) -> int:
    if not SEMVER.match(version):
        print(f"Ungültige Version {version!r}", file=sys.stderr)
        return 2
    mine = tuple(int(part) for part in version.split("."))
    published = [tuple(int(part) for part in m.groups()) for tag in tags if (m := TAG.match(tag))]
    if published and mine <= max(published):
        highest = ".".join(str(part) for part in max(published))
        print(f"Version {version} ist nicht neuer als die höchste veröffentlichte ({highest}).", file=sys.stderr)
        return 1
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("check")
    sub.add_parser("set").add_argument("version")
    sub.add_parser("changelog").add_argument("version")
    newer_parser = sub.add_parser("newer")
    newer_parser.add_argument("version")
    newer_parser.add_argument("tags", nargs="*")
    args = parser.parse_args(argv)

    if args.command == "check":
        return check(args.root)
    if args.command == "set":
        return set_version(args.root, args.version)
    if args.command == "changelog":
        section = changelog_section(args.root, args.version)
        if section is None:
            print(f"CHANGELOG.md hat keinen Abschnitt mit Inhalt für [{args.version}].", file=sys.stderr)
            return 1
        print(section)
        return 0
    return newer(args.version, args.tags)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
