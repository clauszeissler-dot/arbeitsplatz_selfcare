# Signieren & Notarisieren (Apple Developer ID)

Lotse wird **außerhalb des App Stores** verteilt. Der Grund: Die App-Store-Sandbox
verhindert praktisch, dass eine App andere Apps (Ghostty) per AppleScript abfragt.
Mit Developer ID + Notarisierung startet die App trotzdem ohne Gatekeeper-Warnung.

## Einmalig: Zertifikat & Schlüssel

1. **Developer ID Application**-Zertifikat anlegen
   (developer.apple.com → Certificates → „+“ → *Developer ID Application*).
   In der Schlüsselbundverwaltung exportieren als `.p12` mit Passwort.
2. **App Store Connect API-Key** für die Notarisierung
   (appstoreconnect.apple.com → Benutzer und Zugriff → Integrationen → Schlüssel, Rolle *Developer*).
   `.p8` herunterladen, *Key ID* und *Issuer ID* notieren.
3. Signier-Identität ermitteln:
   ```sh
   security find-identity -v -p codesigning
   # → "Developer ID Application: Claus Zeißler (TEAMID1234)"
   ```

## GitHub-Secrets (Repository → Settings → Secrets → Actions)

| Secret | Inhalt |
|---|---|
| `APPLE_CERTIFICATE` | `base64 -i zertifikat.p12 \| pbcopy` |
| `APPLE_CERTIFICATE_PASSWORD` | Passwort der `.p12` |
| `APPLE_SIGNING_IDENTITY` | `Developer ID Application: … (TEAMID)` |
| `KEYCHAIN_PASSWORD` | beliebiges Zufallspasswort |
| `APPLE_API_ISSUER` | Issuer ID |
| `APPLE_API_KEY` | Key ID |
| `APPLE_API_KEY_CONTENT` | kompletter Inhalt der `.p8` |

Danach reicht ein Tag:

```sh
git tag v0.1.0 && git push origin v0.1.0
```

Der Workflow `release.yml` baut eine Universal-App (Apple Silicon + Intel), signiert
mit Hardened Runtime, lässt sie von Apple notarisieren und legt das DMG als
Release-Entwurf auf GitHub ab.

## Lokal signieren (ohne CI)

```sh
export APPLE_SIGNING_IDENTITY="Developer ID Application: … (TEAMID)"
export APPLE_API_ISSUER=… APPLE_API_KEY=… APPLE_API_KEY_PATH=~/keys/AuthKey_XXXX.p8
npm run tauri build -- --target universal-apple-darwin
```

## Was im Bundle steckt

- `Entitlements.plist`: `com.apple.security.automation.apple-events` – nötig, damit
  die Hardened-Runtime-App Apple Events an Ghostty senden darf.
- `Info.plist`: `LSUIElement` (kein Dock-Symbol) und der Text für die
  Automation-Abfrage („Lotse möchte Ghostty steuern“).
- Mikrofon/Kamera: Lotse fragt nur ab, *ob* ein Gerät irgendwo läuft. Es wird nichts
  aufgenommen, deshalb erscheint auch keine Mikrofon-/Kamera-Freigabe.
