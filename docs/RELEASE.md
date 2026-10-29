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


## Lokaler Apple-Freigabeablauf (A-013)

Vorhandene Identität: `Developer ID Application: Claus Zeissler (ZWT6395G7Q)`.
Vorhandenes Schlüsselbundprofil: `KI-Kennzeichnung-Notarization`.
Dieses Profil ist bereits in früheren Projekten eingerichtet und am 2026-10-29
mit einer Apple-History-Abfrage erfolgreich geprüft. Für Lotse sind keine neuen
Zugangsdaten erforderlich. Das Fehlen eines profilspezifischen Namens belegt keinen
fehlenden Apple-Zugang: vor einer Neueinrichtung vorhandene Handoffs prüfen.
Passwörter/Schlüssel nicht auslesen, ausgeben oder in Dateien/CI-Variablen kopieren.

Signierten Produktions-Build erstellen:

```sh
APPLE_SIGNING_IDENTITY='Developer ID Application: Claus Zeissler (ZWT6395G7Q)' npm run tauri build
```

Vorhandenen signierten Stand über den bestehenden Schlüsselbundzugang notarisieren:

```sh
./scripts/notarize.sh
```

Das Skript prüft zuerst Signatur und Zugang, reicht die signierte App als ZIP ein,
sichert Apples Ergebnis und Prüfprotokoll, heftet das Ticket an die App und prüft
Gatekeeper. Danach erstellt es ein DMG aus der bereits mit Ticket versehenen App,
signiert und notarisiert auch dieses und prüft das angeheftete Ticket. Finale
Artefakte und Apple-Protokolle: `target/release/notarized/`. Keine automatische
Veröffentlichung und kein Push. Ein neuer Build benötigt eine erneute Einreichung;
ein früheres Ticket gilt für den damals signierten Stand.

Aktueller Einreichungs-/Prüfstand: [ARBEITSLOGBUCH.md](../ARBEITSLOGBUCH.md).

Quelle: [Apple – Customizing the notarization workflow](https://developer.apple.com/documentation/security/customizing-the-notarization-workflow), am 2026-10-29 mit Ego Lite geprüft.


### Verifizierter Stand 2026-10-29

App und DMG von Apple angenommen, beide Prüfprotokolle ohne Issues. Ticket und
Gatekeeper auch am entpackten ZIP sowie an der App im schreibgeschützt gemounteten
DMG bestätigt. Finale Pakete:

- `target/release/notarized/Lotse.zip` — SHA-256 `57046b66075c8870c4b617e124203932f280a460b046809854f0bb445af0bde3`
- `target/release/notarized/Lotse.dmg` — SHA-256 `05e13130616461f5d429ddde45b37329d4714ba808954757c838709a5c840475`

Das ursprüngliche `target/release/bundle/dmg/Lotse_0.1.0_aarch64.dmg` ist der
signierte Zwischenstand. Für Weitergabe das finale DMG unter `notarized/` verwenden.
Apple-Protokolle liegen im Artefaktordner, Submission-IDs im Projekt-Arbeitslog.
