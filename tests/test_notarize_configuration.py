"""Notarization must require explicit credentials before any external action."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'scripts/notarize.sh'


class NotarizeConfigurationTests(unittest.TestCase):
    def run_script(self, identity=None, profile=None, app=None):
        with tempfile.TemporaryDirectory(prefix='lotse-notary-config-') as directory:
            directory = Path(directory)
            calls = directory / 'calls'
            stub = directory / 'codesign'
            stub.write_text('#!/bin/bash\nprintf "%s\\n" "$@" > "$NOTARY_TEST_CALLS"\nexit 99\n')
            stub.chmod(0o755)
            env = os.environ.copy()
            env.pop('APPLE_SIGNING_IDENTITY', None)
            env.pop('APP_PATH', None)
            env['PATH'] = str(directory) + os.pathsep + env['PATH']
            env['NOTARY_TEST_CALLS'] = str(calls)
            if identity is not None:
                env['APPLE_SIGNING_IDENTITY'] = identity
            if app is not None:
                env['APP_PATH'] = app
            result = subprocess.run(['bash', str(SCRIPT)] + ([] if profile is None else [profile]),
                                    env=env, capture_output=True, text=True)
            return result.returncode, calls.read_text() if calls.exists() else ''

    def test_missing_identity_stops_before_external_action(self):
        code, calls = self.run_script(profile='own-test-profile')
        self.assertEqual(code, 2)
        self.assertEqual(calls, '')

    def test_missing_profile_stops_before_external_action(self):
        code, calls = self.run_script(identity='Developer ID Application: Example (TESTTEAM)')
        self.assertEqual(code, 2)
        self.assertEqual(calls, '')

    def test_explicit_app_is_verified_instead_of_original(self):
        code, calls = self.run_script(identity='Developer ID Application: Example (TESTTEAM)',
                                      profile='own-test-profile', app='/tmp/own-build/Pausenhelfer.app')
        self.assertEqual(code, 99)  # Stub stops before any real keychain or Apple call.
        self.assertIn('/tmp/own-build/Pausenhelfer.app', calls)
        self.assertNotIn('/Lotse.app', calls)


if __name__ == '__main__':
    unittest.main()
