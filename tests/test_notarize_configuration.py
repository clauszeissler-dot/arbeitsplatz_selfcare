"""Notarization must bind the submitted app to explicit signing metadata."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "notarize.sh"
IDENTITY = "Developer ID Application: Example GmbH (TESTTEAM01)"
TEAM_ID = "TESTTEAM01"
BUNDLE_ID = "com.example.pausenhelfer"


class NotarizeConfigurationTests(unittest.TestCase):
    def run_script(
        self,
        *,
        identity=IDENTITY,
        team_id=TEAM_ID,
        bundle_id=BUNDLE_ID,
        profile="own-test-profile",
        app="/tmp/own-build/Pausenhelfer.app",
        actual_identity=IDENTITY,
        actual_team=TEAM_ID,
        actual_bundle=BUNDLE_ID,
    ):
        with tempfile.TemporaryDirectory(prefix="lotse-notary-config-") as directory:
            directory = Path(directory)
            calls = directory / "calls"
            codesign = directory / "codesign"
            codesign.write_text(
                "#!/bin/bash\n"
                'printf "codesign %s\\n" "$*" >> "$NOTARY_TEST_CALLS"\n'
                'if [[ "$*" == *"--display"* ]]; then\n'
                '  printf "Identifier=%s\\nTeamIdentifier=%s\\nAuthority=%s\\n" '
                '"$ACTUAL_BUNDLE_ID" "$ACTUAL_TEAM_ID" "$ACTUAL_SIGNING_IDENTITY" >&2\n'
                "fi\n"
                "exit 0\n"
            )
            codesign.chmod(0o755)
            xcrun = directory / "xcrun"
            xcrun.write_text(
                "#!/bin/bash\n"
                'printf "xcrun %s\\n" "$*" >> "$NOTARY_TEST_CALLS"\n'
                "exit 99\n"
            )
            xcrun.chmod(0o755)
            env = os.environ.copy()
            for key in (
                "APPLE_SIGNING_IDENTITY",
                "APPLE_EXPECTED_TEAM_ID",
                "APPLE_EXPECTED_BUNDLE_ID",
                "APP_PATH",
            ):
                env.pop(key, None)
            env["PATH"] = str(directory) + os.pathsep + env["PATH"]
            env["NOTARY_TEST_CALLS"] = str(calls)
            env["ACTUAL_SIGNING_IDENTITY"] = actual_identity
            env["ACTUAL_TEAM_ID"] = actual_team
            env["ACTUAL_BUNDLE_ID"] = actual_bundle
            if identity is not None:
                env["APPLE_SIGNING_IDENTITY"] = identity
            if team_id is not None:
                env["APPLE_EXPECTED_TEAM_ID"] = team_id
            if bundle_id is not None:
                env["APPLE_EXPECTED_BUNDLE_ID"] = bundle_id
            if app is not None:
                env["APP_PATH"] = app
            args = ["bash", str(SCRIPT)] + ([] if profile is None else [profile])
            result = subprocess.run(args, env=env, capture_output=True, text=True)
            return result, calls.read_text() if calls.exists() else ""

    def test_missing_required_configuration_stops_before_external_action(self):
        cases = (
            {"identity": None},
            {"team_id": None},
            {"bundle_id": None},
            {"profile": None},
        )
        for case in cases:
            with self.subTest(case=case):
                result, calls = self.run_script(**case)
                self.assertEqual(result.returncode, 2)
                self.assertEqual(calls, "")

    def test_explicit_app_is_verified(self):
        result, calls = self.run_script(app="/tmp/own-build/Pausenhelfer.app")
        self.assertEqual(result.returncode, 99)
        self.assertIn("/tmp/own-build/Pausenhelfer.app", calls)
        self.assertNotIn("/Lotse.app", calls)

    def test_matching_metadata_reaches_notarytool(self):
        result, calls = self.run_script()
        self.assertEqual(result.returncode, 99)
        self.assertIn("xcrun notarytool history --keychain-profile own-test-profile", calls)

    def test_wrong_signing_identity_stops_before_apple(self):
        result, calls = self.run_script(actual_identity="Developer ID Application: Attacker (EVILTEAM01)")
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("xcrun", calls)
        self.assertIn("Signieridentität", result.stderr)

    def test_wrong_team_stops_before_apple(self):
        result, calls = self.run_script(actual_team="EVILTEAM01")
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("xcrun", calls)
        self.assertIn("Team-ID", result.stderr)

    def test_wrong_bundle_id_stops_before_apple(self):
        result, calls = self.run_script(actual_bundle="com.example.other")
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("xcrun", calls)
        self.assertIn("Bundle-ID", result.stderr)


if __name__ == "__main__":
    unittest.main()
