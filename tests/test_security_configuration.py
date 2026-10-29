"""Static security contracts for CI, release, and Tauri configuration."""
import base64
import json
import os
from pathlib import Path
import plistlib
import re
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
FULL_SHA = re.compile(r"^[0-9a-f]{40}(?:\s+#.*)?$")


class WorkflowSecurityTests(unittest.TestCase):
    def workflow(self, name):
        return (ROOT / ".github" / "workflows" / name).read_text()

    def test_every_action_is_pinned_to_a_full_commit_sha(self):
        for name in ("ci.yml", "release.yml"):
            for line in self.workflow(name).splitlines():
                if "uses:" not in line:
                    continue
                ref = line.split("uses:", 1)[1].strip().rsplit("@", 1)[-1]
                self.assertRegex(ref, FULL_SHA, f"mutable action in {name}: {line.strip()}")

    def test_release_separates_unprivileged_build_from_approved_signing(self):
        workflow = self.workflow("release.yml")
        self.assertNotIn("\n  push:", workflow)
        self.assertIn("workflow_dispatch:", workflow)
        self.assertIn("build:", workflow)
        self.assertIn("release:", workflow)
        self.assertIn("environment: production-release", workflow)
        self.assertIn("needs: build", workflow)
        self.assertNotIn("tauri-apps/tauri-action", workflow)
        build, release = workflow.split("  release:", 1)
        self.assertNotIn("secrets.", build)
        self.assertIn("APPLE_CERTIFICATE", release)
        self.assertIn("APPLE_EXPECTED_TEAM_ID", release)
        self.assertIn("APPLE_EXPECTED_BUNDLE_ID", release)

    def test_release_validates_version_tag_and_artifact_digest(self):
        workflow = self.workflow("release.yml")
        self.assertIn("scripts/validate-release.sh", workflow)
        self.assertIn("scripts/build-universal.sh", workflow)
        self.assertIn("sha256", workflow.lower())
        self.assertIn("scripts/ci-release.sh", workflow)
        self.assertIn("--draft", (ROOT / "scripts" / "ci-release.sh").read_text())

    def test_release_creates_version_bound_universal_updater_assets(self):
        workflow = self.workflow("release.yml")
        build, release = workflow.split("  release:", 1)
        self.assertNotIn("TAURI_SIGNING_PRIVATE_KEY", build)
        self.assertIn("TAURI_SIGNING_PRIVATE_KEY", release)
        self.assertIn("TAURI_SIGNING_PRIVATE_KEY_PASSWORD", release)

        script = (ROOT / "scripts" / "ci-release.sh").read_text()
        self.assertIn('scripts/create-update-artifacts.sh "$TAG" "$APP"', script)
        for asset in ("Lotse.app.tar.gz", "Lotse.app.tar.gz.sig", "latest.json"):
            self.assertIn(asset, script)

    def test_universal_build_preserves_the_plist_executable_name(self):
        script = (ROOT / "scripts" / "build-universal.sh").read_text()
        self.assertIn("CFBundleExecutable", script)
        self.assertNotIn("Contents/MacOS/Lotse", script)

    def test_universal_release_embeds_frontend_instead_of_using_dev_server(self):
        script = (ROOT / "scripts" / "build-universal.sh").read_text()
        release_builds = [
            line for line in script.splitlines()
            if "cargo build -p lotse" in line and "--release" in line
        ]
        self.assertEqual(len(release_builds), 2)
        for line in release_builds:
            self.assertIn("--features custom-protocol", line)


class TauriSecurityTests(unittest.TestCase):
    def test_capabilities_do_not_grant_network_shell_or_filesystem(self):
        capability = json.loads((ROOT / "src-tauri" / "capabilities" / "default.json").read_text())
        permissions = capability["permissions"]
        forbidden = ("shell:", "fs:", "http:")
        self.assertFalse(any(permission.startswith(forbidden) for permission in permissions))

    def test_csp_has_no_remote_or_connect_sources(self):
        config = json.loads((ROOT / "src-tauri" / "tauri.conf.json").read_text())
        csp = config["app"]["security"]["csp"]
        self.assertNotIn("http:", csp)
        self.assertNotIn("https:", csp)
        self.assertNotIn("connect-src", csp)
        self.assertNotIn("'unsafe-eval'", csp)

    def test_updater_is_pinned_to_signed_official_github_releases(self):
        config = json.loads((ROOT / "src-tauri" / "tauri.conf.json").read_text())
        updater = config["plugins"]["updater"]
        self.assertEqual(
            updater["endpoints"],
            ["https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/latest/download/latest.json"],
        )
        self.assertTrue(updater["requireSignedVersion"])
        self.assertNotIn("dangerousInsecureTransportProtocol", updater)
        self.assertNotIn("PLACEHOLDER", updater["pubkey"])
        decoded_key = base64.b64decode(updater["pubkey"], validate=True).decode()
        self.assertRegex(decoded_key, r"^untrusted comment: minisign public key")

    def test_only_the_cockpit_can_check_and_install_updates(self):
        main = json.loads((ROOT / "src-tauri" / "capabilities" / "default.json").read_text())
        toast = json.loads((ROOT / "src-tauri" / "capabilities" / "toast.json").read_text())
        self.assertEqual(main["windows"], ["main"])
        self.assertEqual(toast["windows"], ["toast"])
        self.assertIn("updater:allow-check", main["permissions"])
        self.assertIn("updater:allow-download-and-install", main["permissions"])
        self.assertIn("process:allow-restart", main["permissions"])
        self.assertNotIn("updater:default", main["permissions"])
        self.assertFalse(any(permission.startswith(("updater:", "process:")) for permission in toast["permissions"]))


class UpdateArtifactTests(unittest.TestCase):
    def test_update_artifact_script_rejects_the_filesystem_root_as_output(self):
        script = (ROOT / "scripts" / "create-update-artifacts.sh").read_text()
        self.assertIn('[[ -z "$OUT" || "$OUT" == "/" ]]', script)

    def test_update_artifact_contract_is_testable_on_linux_ci(self):
        script = (ROOT / "scripts" / "create-update-artifacts.sh").read_text()
        self.assertIn("plistlib", script)
        self.assertNotIn("plutil", script)

    def test_update_manifest_targets_the_universal_bundle_and_embeds_signature(self):
        with tempfile.TemporaryDirectory() as temp:
            temp_path = Path(temp)
            app = temp_path / "Lotse.app"
            (app / "Contents" / "MacOS").mkdir(parents=True)
            (app / "Contents" / "MacOS" / "lotse").write_bytes(b"synthetic universal app")
            with (app / "Contents" / "Info.plist").open("wb") as handle:
                plistlib.dump({"CFBundleShortVersionString": "0.2.0"}, handle)
            out = temp_path / "out"
            bin_dir = temp_path / "bin"
            bin_dir.mkdir()
            npm = bin_dir / "npm"
            npm.write_text(
                "#!/bin/sh\n"
                "for last do :; done\n"
                "printf '%s\\n' 'trusted-comment-signature-for-0.2.0' > \"$last.sig\"\n"
            )
            npm.chmod(0o755)
            env = os.environ.copy()
            env.update({
                "PATH": f"{bin_dir}:{env['PATH']}",
                "TAURI_SIGNING_PRIVATE_KEY": "synthetic-private-key",
                "TAURI_SIGNING_PRIVATE_KEY_PASSWORD": "synthetic-password",
            })

            subprocess.run(
                [str(ROOT / "scripts" / "create-update-artifacts.sh"), "v0.2.0", str(app), str(out)],
                cwd=ROOT,
                env=env,
                check=True,
                capture_output=True,
                text=True,
            )

            archive = out / "Lotse.app.tar.gz"
            signature = out / "Lotse.app.tar.gz.sig"
            manifest = json.loads((out / "latest.json").read_text())
            self.assertTrue(archive.is_file())
            self.assertTrue(signature.is_file())
            self.assertEqual(manifest["version"], "0.2.0")
            self.assertEqual(
                manifest["platforms"],
                {
                    "macos-universal": {
                        "url": "https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/download/v0.2.0/Lotse.app.tar.gz",
                        "signature": "trusted-comment-signature-for-0.2.0",
                    }
                },
            )


if __name__ == "__main__":
    unittest.main()
