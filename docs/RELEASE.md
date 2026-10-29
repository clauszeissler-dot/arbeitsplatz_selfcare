# Signieren und Notarisieren

## Eigene Version veröffentlichen

[Apache-2.0](../LICENSE) erlaubt Änderungen und deren Veröffentlichung.
[BRANDING.md](../BRANDING.md) beschreibt die Abgrenzung zum offiziellen Original.
Veränderte Veröffentlichungen brauchen einen eigenen Namen, eigene
Marken-/App-Grafiken, eine eigene App-Kennung und eigene Apple-Zugänge. Die
Entwickleridentität von Claus Zeißler / KI AffAIrs ist dafür nicht freigegeben.
Das unveränderte offizielle Paket darfst du mit seinen Lizenzhinweisen weitergeben.

1. Eigene Werte für `identifier` und `productName` in `src-tauri/tauri.conf.json`
   setzen und Originalmarke/Icons ersetzen. Beispiele in der [Selbstbau-Anleitung](ANLEITUNG.md#alternativ-selbst-bauen).
2. Für Apple-Freigaben ein eigenes Developer-ID-Zertifikat und ein eigenes
   Notarisierungsprofil im lokalen Schlüsselbund verwenden. Secrets niemals im
   Repository speichern. Ein bestehendes Profil des eigenen Accounts kann
   wiederverwendet werden; dafür kein neues Passwort pro App anlegen.
3. Mit der eigenen Identität bauen, zum Beispiel:

```sh
APPLE_SIGNING_IDENTITY='Developer ID Application: Dein Name (DEINTEAM)' npm run tauri -- build
```

4. Das bereits signierte eigene Paket notarisieren:

```sh
APPLE_SIGNING_IDENTITY='Developer ID Application: Dein Name (DEINTEAM)' \
  APP_PATH="$PWD/target/release/bundle/macos/Pausenhelfer.app" \
  ./scripts/notarize.sh mein-notarisierungsprofil
```

Das Skript verlangt die Identität und das Profil **ausdrücklich**. Es gibt keinen
Default auf Claus' Developer ID oder sein Schlüsselbundprofil. `APP_PATH` wählt
die eigene App; ZIP/DMG übernehmen deren Dateinamen. Ohne `APP_PATH` ist das lokale
Original-Bundle `Lotse.app` der Zielpfad. Der Pfad ist keine Signierberechtigung.
Es werden keine privaten Schlüssel oder Zertifikate mit dem Projekt ausgeliefert.

Das Skript prüft Signatur und Zugang, reicht eine ZIP-Datei bei Apple ein,
liest den Prüfbericht, heftet das Ticket an, prüft Gatekeeper und erstellt danach
ein ebenfalls signiertes/notarisiertes DMG mit der gestapelten App. Es veröffentlicht
keine Dateien im Repository. Bei Weitergabe `LICENSE`, `NOTICE` und die geltenden
Marken-/Identitätshinweise beilegen. Nach Binäränderungen neu signieren/notarisieren;
vorhandene Originaltickets sind keine Freigabe für den neuen Build.

## Was im Bundle steckt

- `Entitlements.plist`: Apple-Events-Berechtigung zum Lesen der Ghostty-Tabs.
- `Info.plist`: Menüleistenmodus und Erklärung der Apple-Events-Nutzung.
- Mikrofon/Kamera: Lotse prüft nur, ob ein Gerät aktiv ist, ohne Aufnahme.

## Lokaler Apple-Freigabeablauf (A-013)

Vorhandene Identität: `Developer ID Application: Claus Zeissler (ZWT6395G7Q)`.
Vorhandenes Schlüsselbundprofil: `KI-Kennzeichnung-Notarization`.
Dieses Profil ist bereits in früheren Projekten eingerichtet und am 2026-10-29
mit einer Apple-History-Abfrage erfolgreich geprüft. Für den offiziellen Lotse-Release
sind keine neuen Zugangsdaten erforderlich. Dieser Zugang ist ausschließlich für
Claus' eigene Original-Releases vorgesehen. Das Fehlen eines profilspezifischen Namens belegt keinen
fehlenden Apple-Zugang: vor einer Neueinrichtung vorhandene Handoffs prüfen.
Passwörter/Schlüssel nicht auslesen, ausgeben oder in Dateien/CI-Variablen kopieren.

Signierten Produktions-Build erstellen:

```sh
APPLE_SIGNING_IDENTITY='Developer ID Application: Claus Zeissler (ZWT6395G7Q)' npm run tauri build
```

Nur für Claus' offizielle Original-Releases: vorhandenen signierten Stand über den bestehenden Schlüsselbundzugang notarisieren:

```sh
APPLE_SIGNING_IDENTITY='Developer ID Application: Claus Zeissler (ZWT6395G7Q)' \
  ./scripts/notarize.sh KI-Kennzeichnung-Notarization
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


## Repository-Download (A-014/A-015)

Das bereits notarisierte Apple-Silicon-Paket wird unverändert als Release
[`lotse-0.1.0`](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/tag/lotse-0.1.0)
angeboten. README und beide Anleitungen verlinken direkt auf `Lotse.dmg`;
zusätzlich werden `Lotse.zip` und `SHA256SUMS` bereitgestellt. Das Repository bleibt
privat, daher setzt der Download entsprechenden GitHub-Zugriff voraus.

Der Tag beginnt bewusst nicht mit `v`: Der bestehende `v*`-Workflow würde einen
separaten Universal-Build erzeugen. Dieser Release enthält exakt den lokal
geprüften ARM64-Build. Veröffentlichter Download und Quellartefakt werden anhand
der SHA-256-Prüfsumme verglichen. Upload/Push erfolgen hier auf ausdrücklichen
Nutzerauftrag; das Notarisierungsskript veröffentlicht weiterhin nichts.

Veröffentlicht am 2026-10-29. Tatsächlicher GitHub-Download aller drei Dateien gegen lokale Prüfsummen verglichen; DMG-Signatur und angeheftetes Ticket auch nach Download erfolgreich geprüft. Direktlink im Remote-README nachgewiesen.

Die Lizenzierung vom 2026-10-29 gilt auch für die unveränderten bereits veröffentlichten Artefakte von `lotse-0.1.0`. Lizenz und Hinweise werden diesem Release separat beigefügt; das DMG/ZIP wird nicht umgepackt, sodass Signatur, Ticket und bisherige Prüfsummen erhalten bleiben. Zukünftige Releases sollen die Lizenzhinweise bereits vor dem Signieren mitliefern.
