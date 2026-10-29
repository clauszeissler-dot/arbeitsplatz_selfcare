"""Static security contracts for CI, release, and Tauri configuration."""
import json
from pathlib import Path
import re
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


if __name__ == "__main__":
    unittest.main()
