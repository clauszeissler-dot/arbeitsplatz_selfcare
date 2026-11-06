"""Versionierung: eine Quelle, ein Bump-Befehl, Changelog und Release-Reihenfolge (A-030)."""
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts" / "versioning.py"
VERSION_FILES = (
    "Cargo.toml",
    "Cargo.lock",
    "package.json",
    "package-lock.json",
    "src-tauri/tauri.conf.json",
)


def run_tool(root, *args):
    return subprocess.run(
        [sys.executable, "-I", str(TOOL), "--root", str(root), *args],
        capture_output=True,
        text=True,
    )


class VersionToolTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        for name in VERSION_FILES:
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy(ROOT / name, target)

    def read(self, name):
        return (self.root / name).read_text()

    def test_repository_versions_agree_in_all_five_places(self):
        result = run_tool(ROOT, "check")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertRegex(result.stdout.strip(), r"^\d+\.\d+\.\d+$")

    def test_check_names_every_place_that_disagrees(self):
        path = self.root / "package.json"
        path.write_text(path.read_text().replace(f'"version": "{self.current()}"', '"version": "9.9.9"', 1))
        lock = self.root / "package-lock.json"
        data = json.loads(lock.read_text())
        data["packages"][""]["version"] = "8.8.8"
        lock.write_text(json.dumps(data, indent=2) + "\n")

        result = run_tool(self.root, "check")

        self.assertNotEqual(result.returncode, 0)
        self.assertIn("package.json", result.stderr)
        self.assertIn("package-lock.json", result.stderr)

    def current(self):
        return run_tool(ROOT, "check").stdout.strip()

    def test_set_updates_all_five_places_and_nothing_else(self):
        before_lock = json.loads(self.read("package-lock.json"))
        cargo_lock_before = self.read("Cargo.lock")

        result = run_tool(self.root, "set", "7.8.9")

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(run_tool(self.root, "check").stdout.strip(), "7.8.9")
        after_lock = json.loads(self.read("package-lock.json"))
        self.assertEqual(after_lock["name"], before_lock["name"])
        self.assertEqual(after_lock["packages"][""]["version"], "7.8.9")
        # Abhängigkeiten behalten ihre eigenen Versionen
        for key, value in before_lock["packages"].items():
            if key:
                self.assertEqual(after_lock["packages"][key], value, key)
        cargo_lock_after = self.read("Cargo.lock")
        changed = [
            (a, b)
            for a, b in zip(cargo_lock_before.splitlines(), cargo_lock_after.splitlines())
            if a != b
        ]
        self.assertEqual(len(changed), 2, changed)
        for _, new in changed:
            self.assertEqual(new, 'version = "7.8.9"')
        self.assertEqual(len(cargo_lock_before.splitlines()), len(cargo_lock_after.splitlines()))

    def test_set_rejects_formats_other_than_plain_semver_without_touching_files(self):
        snapshot = {name: self.read(name) for name in VERSION_FILES}
        for bad in ("1.2", "v1.2.3", "1.2.3-dev", "01.2.3", "1.2.3.4", ""):
            with self.subTest(bad=bad):
                result = run_tool(self.root, "set", bad)
                self.assertNotEqual(result.returncode, 0)
                for name, text in snapshot.items():
                    self.assertEqual(self.read(name), text, name)

    def test_changelog_section_is_extracted_for_the_release_notes(self):
        (self.root / "CHANGELOG.md").write_text(
            "# Änderungsprotokoll\n\n## [Unveröffentlicht]\n\n- Später\n\n"
            "## [0.3.0] – 2026-11-01\n\n### Neu\n\n- Laufzeit-Uhren\n- Tag beenden\n\n"
            "## [0.1.0] – 2026-10-29\n\n- Erster Release\n"
        )

        result = run_tool(self.root, "changelog", "0.3.0")

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("Laufzeit-Uhren", result.stdout)
        self.assertIn("Tag beenden", result.stdout)
        self.assertNotIn("Erster Release", result.stdout)
        self.assertNotIn("Später", result.stdout)

    def test_release_without_changelog_section_is_rejected(self):
        (self.root / "CHANGELOG.md").write_text("# Änderungsprotokoll\n\n## [0.1.0] – 2026-10-29\n\n- Erster Release\n")
        self.assertNotEqual(run_tool(self.root, "changelog", "0.3.0").returncode, 0)
        (self.root / "CHANGELOG.md").write_text("# Änderungsprotokoll\n\n## [0.3.0] – 2026-11-01\n\n")
        self.assertNotEqual(run_tool(self.root, "changelog", "0.3.0").returncode, 0, "leerer Abschnitt")

    def test_a_release_must_be_newer_than_every_published_tag(self):
        self.assertEqual(run_tool(self.root, "newer", "0.3.0", "lotse-0.1.0", "v0.2.0").returncode, 0)
        self.assertEqual(run_tool(self.root, "newer", "0.10.0", "v0.9.0").returncode, 0, "numerisch, nicht textuell")
        self.assertNotEqual(run_tool(self.root, "newer", "0.2.0", "lotse-0.1.0", "v0.2.0").returncode, 0)
        self.assertNotEqual(run_tool(self.root, "newer", "0.1.5", "v0.2.0").returncode, 0)
        self.assertEqual(run_tool(self.root, "newer", "0.1.0").returncode, 0, "erster Release")
        self.assertEqual(run_tool(self.root, "newer", "0.3.0", "irgendein-branch").returncode, 0, "Fremdtags ignorieren")


class ReleaseContractTests(unittest.TestCase):
    def script(self, name):
        return (ROOT / "scripts" / name).read_text()

    def test_validate_release_uses_the_shared_version_tool(self):
        text = self.script("validate-release.sh")
        for command in ("check", "changelog", "newer"):
            self.assertRegex(text, rf'versioning\.py"? {command}\b')

    def test_release_notes_come_from_the_changelog(self):
        text = self.script("ci-release.sh")
        self.assertRegex(text, r'versioning\.py"? changelog\b')
        self.assertNotIn("Signiert und von Apple notarisiert. DMG öffnen", text)

    def test_only_the_release_workflow_marks_builds_as_release(self):
        workflow = (ROOT / ".github" / "workflows" / "release.yml").read_text()
        self.assertRegex(workflow, r"LOTSE_BUILD:\s*release")
        self.assertNotIn("LOTSE_BUILD", (ROOT / ".github" / "workflows" / "ci.yml").read_text())
        self.assertRegex(self.script("build-universal.sh"), r'LOTSE_BUILD="\$\{LOTSE_BUILD:-ci\}"')

    def test_local_build_marks_itself_and_installs_into_applications(self):
        text = self.script("build-local.sh")
        self.assertRegex(text, r"LOTSE_BUILD=\"?local-")
        self.assertIn("git rev-parse --short", text)
        self.assertIn("/Applications/Lotse.app", text)
        self.assertIn("plutil -extract", text, "Autostart-Pfad wird nur gelesen und gemeldet")
        for forbidden in ("plutil -replace", "plutil -insert", "launchctl", "defaults write"):
            self.assertNotIn(forbidden, text, "Autostart-Datei wird nie umgeschrieben")
        self.assertNotIn("Developer ID Application:", text, "keine feste Entwickleridentität im Repository")

    def test_set_version_wrapper_exists_and_is_executable(self):
        path = ROOT / "scripts" / "set-version.sh"
        self.assertTrue(path.exists())
        self.assertTrue(path.stat().st_mode & 0o111)

    def test_changelog_documents_the_published_release(self):
        text = (ROOT / "CHANGELOG.md").read_text()
        self.assertRegex(text, r"## \[0\.1\.0\]")
        self.assertRegex(text, re.compile(r"## \[Unveröffentlicht\]|## \[0\.3\.0\]"))


if __name__ == "__main__":
    unittest.main()
