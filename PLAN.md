# Aktueller Arbeitsplan

## Sichere Updates aus GitHub Releases · A-025

**Sichtbares Ergebnis / Zielort:** Eine updaterfähige Lotse-App zeigt im Cockpit
den Updatezustand, kann manuell nach einer neueren offiziellen GitHub-Version
suchen und installiert sie erst nach Zustimmung und erfolgreicher
kryptografischer Prüfung. Der Releasepfad erzeugt die dafür nötigen signierten
Universal-Artefakte und Metadaten. Zielorte sind der Feature-Branch, ein lokaler
synthetischer Update-Feed und ein echter Universal-App-Build; kein neuer
öffentlicher Release.

**Abschlussnachweis:** beobachtete Red/Green-Tests für UI-Zustände und
Releasevertrag; S1-Missbrauchstest lehnt manipuliertes/unsigniertes Paket ab,
während das korrekt signierte Kontrollpaket akzeptiert wird; vollständige
Rust-/Frontend-/Python-/Securitytests, Formatierung, Clippy und Universal-Build;
Updater-Konfiguration enthält nur öffentlichen Schlüssel und festen offiziellen
HTTPS-Endpunkt. Dokumentation benennt den einmaligen manuellen Sprung von
`0.1.0`.

**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T15:39:27+01:00 /
2026-10-29T16:01:42+01:00.

**Sicherheitsgate S1:** Autorisiert sind lokaler Code, synthetische lokale
Updateartefakte und der eigene GitHub-Releasepfad. Keine Produktionseinreichung,
kein echter Release und keine Verwendung/Anzeige privater Schlüssel. Invarianten:
fest gebundener offizieller HTTPS-Feed; Update erst nach Signaturprüfung und
Nutzerzustimmung; fehlende/fehlerhafte Metadaten bleiben sichtbar fehlerhaft und
verändern die installierte App nicht; privater Update-Schlüssel nur als
geschütztes Release-Secret; Apple-Signierung/Notarisierung nicht durch die
Updater-Signatur ersetzen.

1. [x] Offizielle Tauri-v2-Updater-Spezifikation ausschließlich mit Ego Lite
   prüfen; bestehenden UI-, Rust- und Releasepfad zuordnen.
2. [x] Funktionale Red-Tests für Updatezustände/Bedienung und Releaseartefakte
   schreiben und mit erwarteter Ursache scheitern sehen.
3. [x] Minimalen Updater, Cockpit-Bedienung, feste Feed-/Public-Key-Konfiguration
   und sicheren Release-Artefaktpfad implementieren; Dokumentation ergänzen.
4. [x] S1 SECURITY RED/GREEN: legitimes signiertes Kontrollupdate und
   manipulierte/unsignierte Gegenprobe mit unverändertem Zielzustand ausführen.
5. [x] Gesamttests, Universal-Build und lokale Zielprüfung; Anforderungen,
   Projekt-/Global-Log aktualisieren, gezielt committen und über geschützten PR
   integrieren. Kein Tag und kein Release.

**Zielabgleich 2026-10-29T16:01:42+01:00:** Schritte 1–4 sind mit dem
Produktionsschlüssel, einem echten Universal-Archiv und negativer Manipulations-
sowie Versionsbindungsprobe belegt. Die vollständigen lokalen Gates waren grün;
nach der letzten Ausgabepfad-Härtung laufen sie erneut. GitHub-CLI-Zugang ist
abgelaufen und blockiert derzeit nur Environment-Secrets, PR und Remote-Kontrolle.
Kein Release wurde angelegt.

**Nächster Schritt:** finale Gates und lokaler Commit; danach GitHub-Zugang
erneuern, die zwei Updater-Secrets hinterlegen und den geschützten PR integrieren.

**CI-Korrektur 2026-10-29T16:13:55+01:00:** PR #13 deckte im verpflichtenden
Ubuntu-Securityjob auf, dass der neue Artefaktvertrag macOS-`plutil` voraussetzte.
Die Plist-Prüfung verwendet jetzt portables Python-`plistlib`; ein neuer
Regressionstest verbietet `plutil` in diesem Skript. Lokal 19/19 grün, erneuter
CI-Lauf folgt.

**Abschluss 2026-10-29T16:28:15+01:00:** PR #13 nach grünen Pflichtchecks
gemergt (`00d5dd0`): Kernlogik/Security 3:44 Minuten, vollständiger
Universal-Mac-Build 12:36 Minuten. Updater-Secrets im geschützten Environment
vorhanden; Apple-Secrets bleiben absent und der Releasepfad fail-closed. Der
Remote-Feature-Branch wurde gelöscht. Kein neuer Tag, Draft oder Release; der
einzige veröffentlichte Stand bleibt `lotse-0.1.0`.

**Nächster Schritt:** Erst bei einem gesondert beauftragten `0.2.0`-Release die
Apple-Environment-Secrets einrichten, Releaseworkflow manuell freigeben, Draft
prüfen und anschließend bewusst veröffentlichen. `0.1.0` muss einmalig manuell
auf diese updaterfähige Version wechseln.

---

## Stabilität, Farblogik und Zeitübersicht · A-021 bis A-023

### Reale Reproduktion und Produktionsbuild-Korrektur · A-024

**Korrektur 2026-10-29T14:57:33+01:00:** Die erneute Nutzerreproduktion widerlegte den ersten Abschluss als Ursache. JavaScript konnte weder Fehleransicht noch Diagnose schreiben, weil beide WebViews bereits beim Dokumentladen mit WebKit `-1004` scheiterten. `scripts/build-universal.sh` verwendete plain `cargo build` ohne Tauri-Feature `custom-protocol`; die gebündelte App versuchte deshalb `devUrl` auf Port 1420 zu laden. Sounds/Engine liefen nativ weiter. Das Wegklicken war nicht ursächlich.

1. [x] Roter statischer Regressionstest verlangt `--features custom-protocol` für ARM64 und x86_64.
2. [x] Feature im App-Crate definiert und im Universal-Build für beide Architekturen aktiviert; Entwicklerdokumentation ergänzt.
3. [x] Vollständige lokale Gates: 43 Rust-, 5 UI- und 13 Python-/Securitytests, Build, Formatierung und Clippy grün.
4. [x] Universal-Build `x86_64 arm64`, Developer-ID-Signatur, Apple Accepted für App/DMG, Ticket und Gatekeeper.
5. [x] Dev-Server beendet; final installierte `/Applications/Lotse.app` zeigt feste Cockpit- und Zeitleistenüberschriften (322 Accessibility-Knoten), keine WebKit-`-1004`-Fehler.
6. [x] Veröffentlichte `lotse-0.1.0` separat aus GitHub-DMG schreibgeschützt gestartet: kein `-1004`; bestehender Release ist nicht von diesem später eingeführten Universal-Buildfehler betroffen.

**Nächster Schritt:** Hotfix über geschützten PR integrieren. Kein neuer Release nötig, solange kein neuer Universal-Download veröffentlicht wird; jeder künftige Universal-Release läuft durch den neuen Buildtest.

**Sichtbares Ergebnis / Zielort:** Das lokale Dashboard bleibt nach wiederholtem Ignorieren/Schließen von Hinweisen funktionsfähig, Fokus und Pause folgen einer einheitlichen semantischen Farblogik, und eine kompakte Tages-Zeitleiste zeigt Startzeit/aktive Dauer erkannter Tasks sowie Beginn/Ende/Dauer von Pausen.
**Abschlussnachweis:** belegte Fehlerursache oder ausdrücklich dokumentierte Nicht-Reproduzierbarkeit mit Diagnostik; Red/Green-Regressionen für Hinweis-Lebenszyklus und Zeitmodell; vollständige Rust-/Frontend-/Securitytests; lokaler signierter/notarisierter App-Build tatsächlich unter `/Applications` aktiv; Laufzeit- und Datenrechteprüfung. Browserprüfungen ausschließlich Ego Lite, falls für die Oberfläche nötig.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T14:22:45+01:00 / 2026-10-29T14:37:04+01:00.
**Gates:** erst Logs, Crashberichte und Zustandsfluss auswerten, dann Ursache/Hypothese testen; ignorierte Hinweise nicht vorschnell als Absturzursache ausgeben; Zeitdaten nur lokal, begrenzt und ohne Rohpfade/Prozessargumente; bestehende drei fremde AUTO-Handoffs unangetastet; kein GitHub-Release oder Release-Tag.

1. [x] A-021: Abgebrochenen Notarisierungsstand, Prozess-/Launchzustand, macOS-Unified-Logs, Crashreports und Hinweis-/Webview-Lifecycle erfassen; reproduzierbare Hypothese bilden.
2. [x] A-021: Minimalen roten Regressionstest für die belegte Ursache und wiederholtes Verwerfen/Ignorieren von Hinweisen erstellen; gezielte Wiederherstellung/Diagnostik implementieren.
3. [x] A-022: Alle Fokus-/Pause-Farbrollen und UI-Komponenten zuordnen; semantische Tokens und einen statischen/visuellen Regressionstest zuerst rot anlegen, dann vereinheitlichen.
4. [x] A-023: Datenschutzarmes Tages-Zeitmodell testgetrieben ergänzen: erkannte Tasks mit erstem Start/aktiver Dauer, Pausenereignisse mit Start/Ende/Dauer; Dashboard kompakt und zugänglich darstellen.
5. [x] Gesamttests/Build, Ego-Lite-TaskSpace, lokalen Developer-ID-/Apple-Ablauf abschließen, App unter `/Applications` aktualisieren und tatsächlichen Prozess, Dashboard, Schreibzyklen und Dateirechte prüfen.

**Erster Abschluss 2026-10-29T14:37:04+01:00, durch A-024 korrigiert:** Kein Prozesscrash; Event-/Start-Recovery wurde gehärtet. Die Aussage, dies erkläre den realen Ausfall vollständig, war falsch. Die konkrete Ursache und finale Abnahme stehen im A-024-Abschnitt oben.
**Remote-Abschluss 2026-10-29T14:46:18+01:00:** PR #11 nach grünen Pflichtchecks gemergt; Squash-Commit `c78209e`. `Kernlogik und Security` 2:23 Minuten, `Mac-App bauen (unsigniert)` 6:46 Minuten. Kein Tag und kein GitHub-Release.
**Nächster Schritt:** Realbetrieb beobachten. Falls ein WebView-Fehler wiederkehrt, die feste Kategorie und den Zeitstempel aus `ui-diagnostics.json` mit dem sichtbaren Zustand abgleichen; keine privaten Rohdaten erforderlich.

---

## Sicherheitshärtung · A-020

**Sichtbares Ergebnis / Zielort:** lokaler, vollständig geprüfter Härtungsstand im Repository sowie ein PR-fähiger GitHub-Branch; nach autorisiertem Push ein geschütztes öffentliches Repository mit PR-Pflicht, Sicherheitsfunktionen und manuell freizugebender Release-Umgebung. Kein neuer Produktrelease.
**Abschlussnachweis:** Regressionstests für die drei Findings, vollständige Workspace-/Frontend-/Security-Prüfungen, macOS-Universal-Build, überprüfte GitHub-Regeln/Einstellungen und Anforderungsabgleich.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T10:58:32+01:00 / 2026-10-29T12:21:52+01:00.
**Gates:** bestehende ungetrackte Handoff-Dateien unangetastet lassen; keine vorhandenen Release-Artefakte verändern; Apple-Secrets niemals auslesen oder protokollieren; vor GitHub-Änderungen lokale Prüfungen abschließen; kein Release-Tag und keine Veröffentlichung erzeugen.
**Sicherheitsgate:** S1 für Persistenz-, Signier- und Deployment-Grenzen. Invarianten: ältere Zustände überschreiben keine neueren; lokale Dateien sind nur für den Benutzer zugänglich; falsche Signieridentität/Team-/Bundle-ID stoppt vor Apple; kein veränderlicher Action-Code erhält Release-Zugangsdaten.

1. A-020/F1: Tests für unveränderliche Action-Referenzen, Geheimnisgrenzen und geschütztes Release-Environment zuerst rot ausführen; Release-Workflow auf getrennten Build-/Signierpfad mit minimalen Rechten umstellen.
2. A-020/F2: Persistenztests für Reihenfolge, atomare Dateien, Rechte, Symlinks und beschädigte Daten zuerst rot ausführen; zentralen Persistenzpfad implementieren und Fehler sichtbar machen.
3. A-020/F3: Signer-/Team-/Bundle-Prüfungen im Notarisierungsskript zuerst rot ausführen; vor jeder Einreichung verbindlich prüfen und temporäre Secrets bereinigen.
4. A-020/Härtung: Prozessausgaben begrenzen, Tauri-Konfiguration absichern, RustSec/npm-Prüfungen, Dependabot und SECURITY.md ergänzen.
5. Gesamtdiff unabhängig prüfen, vollständige Tests/Builds ausführen, Anforderungen und Logs aktualisieren, gezielt committen; danach autorisierten GitHub-Branch/PR und Repository-Regeln einrichten und remote kontrollieren.

**Zielabgleich 2026-10-29T11:32:58+01:00:** Die drei Findings sind implementiert und ihre fokussierten Red/Green-Tests bestanden. Der unabhängige Nachreview führte zusätzlich zur Entfernung des automatischen Tag-Triggers und zur sichtbaren Behandlung fehlgeschlagener UI-Aktionen. Der Universal-Build ist nach Trennung von direktem Cargo-Build und Tauri-Bundling nachweislich `x86_64 arm64`. Zum sichtbaren Ergebnis fehlen jetzt Gesamttest, lokale Datenmigration, GitHub-Regeln/PR und Remote-Kontrolle.
**Abschluss 2026-10-29T11:51:38+01:00:** A-020 geprüft. PR [#1](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/pull/1) nach grünen Pflichtchecks gemergt (`9f95ac8`). `main` verlangt PR, aktuelle Basis und beide Checks; Admins sind eingeschlossen, Force-Push/Löschung gesperrt. Actions akzeptiert nur GitHub-eigene, vollständig SHA-gepinnte Actions. `production-release` verlangt Claus' manuelle Freigabe und geschützte Branches. Secret Scanning, Push Protection, Dependabot-Sicherheitsupdates/-Alerts und private Meldungen sind aktiv. Die vorhandenen lokalen Lotse-Dateien sind `0600`, ihre Ordner `0700`. Kein Tag/Release erzeugt. RustSec ohne blockierende Vulnerability, mit zwei dokumentierten transitiven Warnungen. Dependabot-Alert #1 zu `glib 0.18.5` wurde als ausschließlich ungenutzter Linux/GTK-Zweig belegt und begründet als `not_used` geschlossen; bei Linux-Support oder Tauri-Upstream-Update neu bewerten. GitHubs zusätzliche Secret-Validity-Checks bleiben für dieses Repository deaktiviert; die verlangten Kernkontrollen sind aktiv.
**Betriebsrest:** Das Release-Environment enthält noch keine Apple-Secrets und blockiert daher sicher. Vor dem ersten neuen offiziellen Release die in `docs/RELEASE.md` genannten Werte einmalig als Environment-Secrets hinterlegen; keine lokalen Schlüssel auslesen oder ins Repository kopieren.
**Status:** geprüft abgeschlossen; keine offene Code- oder Repository-Härtung aus A-020.
**Nachkontrolle 2026-10-29T12:21:52+01:00:** Die noch laufende alte Entwicklungsinstanz setzte `state.json` erneut auf `0644`. Sie wurde gezielt beendet und durch den neuen lokal signierten Universal-Build ersetzt; nach mehreren tatsächlichen Schreibzyklen bleibt `state.json` `0600`. Das Signaturgate deckte außerdem eine Groß-/Kleinschreibungsabweichung auf: `Info.plist` nennt `lotse`, das Skript hatte `Lotse` hart codiert. Folge-PR #10 liest und validiert `CFBundleExecutable` beider Architekturen. Regressionstest rot/grün, realer Universal-Build `x86_64 arm64`, Developer-ID-Signatur und strikte Prüfung erfolgreich. Kein Release und keine Notarisierung.

---

## Sicherheitsaudit · A-019

**Sichtbares Ergebnis / Zielort:** versiegelter Codex-Security-Bericht für den lokalen Repository-Stand sowie ein verständlicher Befund zu lokaler Speicherung, Zugriff, Verschlüsselung und Release-Sicherheit.
**Abschlussnachweis:** Scan-ID `4eb909c1-564f-4aaa-8d1c-a7a978d4af62`, erzeugter `report.md`, drei validierte Findings, Dateirechte am tatsächlichen App-Datenpfad, GitHub-Sicherheitskonfiguration, npm-Audit und Apple-Signatur-/Ticketprüfung.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T10:24:00+01:00 / 2026-10-29T10:41:27+01:00.
**Gates:** Prüfung und Dokumentation; keine Codekorrektur, GitHub-Konfigurationsänderung, Veröffentlichung oder Push beauftragt. Quellstand während des Scans unverändert. Daybreak-Advisory des Plugins widersprach der bestätigten Nutzerfreigabe und war kein Scan-Gate.

1. Architektur, Datenpfade, Berechtigungen, Befehls-/AppleScript-Grenzen und Release-Kette quellgestützt erfassen.
2. Unabhängigen Baseline- und fokussierte Teilreviews zusammenführen; Findings gegen Gegenbelege validieren.
3. Tatsächliche App-Datenrechte, aktuelle GitHub-Kontrollen, npm-Advisories und vorhandene Release-Signatur/Tickets prüfen.
4. Semantischen Bericht versiegeln und Anforderungen/Logs aktualisieren.

**Abschluss:** 1 hohes Finding (mutable GitHub Actions mit Release-Secrets) und 2 niedrige Findings (persistente Zustands-Race, fehlende Signer-Bindung im lokalen Notarisierungsskript). App-Daten liegen als unverschlüsseltes JSON unter `~/Library/Application Support/de.kiaffairs.lotse/`; die übergeordneten Ordner sind auf diesem Mac `0700`, Rohdaten aus Ghostty/Prozessargumenten werden nicht dauerhaft gespeichert. Keine eingebetteten Secrets, kein belegter XSS-/Command-Injection-Pfad; npm-Audit 0 Findings. Aktueller RustSec-Abgleich und GitHub-Härtung bleiben als konkrete Folgepunkte offen.
**Nächster Schritt:** Nutzer entscheidet, ob Findings behoben und GitHub-Schutzfunktionen aktiviert werden sollen; danach RustSec/Dependabot neu prüfen.

---

## Lizenz und eigene Entwickleridentität · A-017 / A-018

**Sichtbares Ergebnis / Zielort:** GitHub main enthält LICENSE/NOTICE und klare Regeln für Originalweitergabe versus eigene Builds; README und Selbstbau-/Release-Anleitung verlinken sie. Bestehender Release erhält die Lizenz als separate Datei, sein notarisiertes Paket bleibt unverändert.
**Abschlussnachweis:** vollständiger Apache-2.0-Lizenztext, passende Paket-Metadaten, eigene App-Kennung im Selbstbau-Beispiel, kein stiller Rückgriff des Signierskripts auf Claus' Identität; lokale Prüfungen und Remote-Dateien/Release-Anhänge nach Push prüfen.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T10:06:05+01:00.
**Entscheidung:** Nutzer erlaubt Veröffentlichung von Änderungen unter eigener Identität. Apache-2.0 für Code/Dokumentation/eigene Klänge, Marken/Logo und Entwickleridentität nicht mitlizenziert; unveränderte Originalpakete dürfen mit vorhandener Signatur geteilt werden.
**Umfang/Freigabe:** Lizenz/Anleitungen und gezielte Anpassung des Notarisierungsskripts; Commit/Push und Lizenz-Anhang zum bestehenden Release im Rahmen der beauftragten Repository-Veröffentlichung. Keine Änderung/Re-Signierung des existierenden Downloads, keine neue Nutzung privater Schlüssel.

1. A-017: Standardlizenz und NOTICE einführen, Umfang/Markenrichtlinie dokumentieren, Metadaten und README aktualisieren.
2. A-018: Selbstbau mit eigener Bundle-ID/Produktkennung zeigen; Signierskript verlangt explizite eigene Identität und Profil statt eingebauter Claus-Vorgaben. Bestehenden lokalen Owner-Zugang weiterhin dokumentieren und nur ausdrücklich nutzen.
3. A-017/A-018: Skript-Fehlerfälle vor externen Aktionen testen, Cargo/Node-Metadaten und Links prüfen, committen/pushen; separate Lizenz-/Hinweis-Dateien am Release anbieten und remote prüfen.

Zielabgleich 2026-10-29T10:12:21+01:00: LICENSE/NOTICE/BRANDING.md und Metadaten angelegt. Drei Tests für fehlende explizite Identität/Profil sowie eigenen App-Pfad waren rot und sind grün. Bau-/Release-Anleitungen angepasst. Zum Ziel fehlen Commit/Push, drei separate Release-Anhänge und Remote-Abgleich.

Abschluss 2026-10-29T10:15:07+01:00: A-017/A-018 geprüft. Commit `1297a97` auf GitHub main. LICENSE/NOTICE/BRANDING.md sowie README, Cargo-Metadaten und Skript remote bytegleich geprüft. Alle drei Lizenzanhänge vom bestehenden Release heruntergeladen und identisch; Release-Notiz ordnet Lizenz ausdrücklich dem ursprünglichen Paket zu. DMG-/ZIP-Digests auf GitHub unverändert. Drei Konfigurationstests grün; gleicher Befehl in bestehender CI ergänzt.

Status: erledigt. Nächster Einstieg: Bei künftigen Releases Lizenz-/Herkunftshinweise vor dem Signieren beilegen; Original-Owner explizit mit eigener Identität und vorhandenem eigenem Profil, Forks mit eigenen Werten. Keine Änderung/Neu-Notarisierung des bestehenden Pakets erforderlich.

---

## README-Funktionsabgleich · A-016

**Sichtbares Ergebnis / Zielort:** README auf GitHub main beschreibt die tatsächlich ergänzten Funktionen vollständig: Timer/Augenpause, Klänge, Haltungen, aktivitätsbasierter Rhythmus, Dock/⌘Tab und Download.
**Abschlussnachweis:** lokale Inhalts-/Linkprüfung und nach Push Remote-README auf die ergänzten Angaben prüfen.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T09:49:22+01:00.
**Umfang/Freigabe:** Abschluss der bereits beauftragten README-Aktualisierung; Dokumentation committen/pushen, Binärpakete unverändert. Keine neuen Build-/Browserprüfungen für reine Textkorrektur.
**Abschluss:** A-016 geprüft. README per GitHub-API auf main gelesen, bytegleich zur lokal geprüften Fassung. Alle ergänzten Funktionsangaben und lokalen Dateilinks bestätigt. Commit `2cdf97c` übertragen.
**Nächster Schritt:** README bei künftigem Funktionsupdate gemeinsam mit der Umsetzung nachführen.

---

## Repository-Download · A-014 / A-015

**Sichtbares Ergebnis / Zielort:** bestehendes GitHub-Repository `clauszeissler-dot/arbeitsplatz_selfcare`: Release `lotse-0.1.0` mit unverändert notarisiertem Apple-Silicon-DMG/ZIP und direktem Download-Link oben im README.
**Abschlussnachweis:** Release veröffentlicht, Asset erneut authentifiziert heruntergeladen und SHA-256 mit geprüfter Quelle identisch; Remote-README-Link stimmt mit Release-Asset-URL überein.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T09:00:00+01:00.
**Freigabe:** Nutzerauftrag umfasst Upload, README und dafür notwendigen Push. Repository bleibt privat; keine Sichtbarkeitsänderung.
**Gates:** bestehende Signatur-/Notarisierungsbelege wiederverwenden, finalen Download prüfen. Release-Tag `lotse-0.1.0` statt `v*`, damit die bestehende Universal-Build-Automation das bereits geprüfte ARM64-Paket nicht neu baut. Kein neuer Binärbuild.

1. A-014: bestehende Remote/Release-Lage erfassen, Hashes und Apple-Silicon-Architektur bestätigen; Paket als Release-Entwurf hochladen.
2. A-015: Download-Link und Installationshinweis im README und den bestehenden Anleitungen ergänzen; frühere Aussagen zu fehlenden Downloads entfernen.
3. A-014/A-015: eigene Änderungen committen/pushen, Release veröffentlichen; Remote-Datei herunterladen und README-Link am Ziel nachweisen.

Zielabgleich 2026-10-29T09:01:17+01:00: Release-Erstellung verlangt einen bereits remote vorhandenen Commit; daher erst Commit/Push, dann Release-Upload. Clippy-Pflichtprüfung meldet field_reassign_with_default in bestehendem Migrations-Test; nur Testinitialisierung korrigieren, keine Produkt-/Binäränderung.

Abschluss 2026-10-29T09:04:10+01:00: A-014/A-015 geprüft erfüllt. Commit f1244a5 und vorherige geprüfte Umsetzung auf main übertragen. Release lotse-0.1.0 veröffentlicht; tatsächlicher GitHub-Download für DMG/ZIP/SHA256SUMS identisch zur Quelle, heruntergeladenes DMG besteht Signatur-/Ticketprüfung. Remote-README-Link stimmt mit Asset-URL überein. Core-Tests 31/31 und Clippy grün.

Status: erledigt. Repository bleibt privat, Download Apple Silicon/macOS 13+. Separater GitHub-CI-Universal-Build läuft noch; Core-CI bereits erfolgreich. Freigegebenes ARM64-Paket stammt aus dem zuvor lokal vollständig geprüften/notarisierten Build. Nächster Schritt erst beim nächsten Produktupdate: neuen Build signieren/notarisieren und als eigenen Release einstellen.

---

## Wiederaufnahme A-013 · 2026-10-29T08:44:19+01:00

**Sichtbares Ergebnis / Zielort:** Apple-notarisierte Lotse.app sowie ZIP und DMG unter `target/release/notarized/` und Ticket am lokalen App-Bundle.
**Abschlussnachweis:** Apple Accepted, Logs, stapler validate, codesign und Gatekeeper; ZIP nach Entpacken und DMG prüfen.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T08:44:19+01:00.
**Befund und Entscheidung:** Nutzerkorrektur bestätigt; vorhandenes Profil `KI-Kennzeichnung-Notarization` aus früheren Projektdokumenten gefunden. Apple-History-Abfrage authentifiziert erfolgreich. Kein neuer Zugang erforderlich. Frühere Blockerannahme ist aufgehoben.
**Abschluss / letzter Zielabgleich:** 2026-10-29T08:46:44+01:00. A-013 geprüft: Apple Accepted für App und DMG, Logs ohne Issues; Tickets, Signaturen und Gatekeeper auch im entpackten ZIP und App im schreibgeschützt gemounteten DMG grün. Zielartefakte unter `target/release/notarized/`. Keine weitere Freigabe oder Zugangseinrichtung erforderlich. Vorhandene 33 Tests, Ego-Lite- und native Belege gelten weiter; keine Binäränderung.
**Nächster Schritt:** Bei künftigem Produktupdate neu bauen und mit dem bestehenden Profil erneut notarisieren. Dieser Auftrag ist technisch abgeschlossen; persönliche Klangbewertung durch Claus steht weiterhin aus.

## Vorheriger Arbeitslauf 2026-10-29 · A-009 / A-010 (Stand vor Nutzerkorrektur)

**Sichtbares Ergebnis:** Zeitbezogene Übungshinweise bieten Starten, Countdown und Abschluss; eigene sanfte Klänge ersetzen Glass.
**Zielort:** lokales `target/release/bundle/macos/Lotse.app`, Browserprüfung der Vorschau ausschließlich Ego Lite.
**Abschlussnachweis:** Timer-/Hinweis-Regressionstests, Frontend- und Tauri-Build, Ego-Lite-Nutzerablauf und gebündelte WAV-Dateien. Klanggeschmack bleibt durch Claus zu beurteilen.
**Arbeitsbeginn / letzter Zielabgleich:** 2026-10-29T08:16:07+01:00 / 2026-10-29T08:25:03+01:00.
**Gates:** kein Push, keine Veröffentlichung oder Installation; lokales Bundle wie bisher.

1. A-009: strukturierte Übungsdauer für Augen- und Pausenhinweise; nativer Timer unabhängig vom Webview; keine Verdrängung laufender Timer.
2. A-010: eigene zusammengehörige, lokal komponierte WAV-Klänge; Hinweis-Schalter erhalten, bewusst gestarteter Timer mit eigenem abschaltbaren Abschlusston und Klangprobe.
3. A-009/A-010: passende Tests, Ego-Lite-Prüfung, Produktions-Bundle, Dokumentation und gezielter lokaler Commit.

4. A-011/A-012: Arbeitslog laufend führen; Dashboard beim Öffnen im Dock/⌘Tab anzeigen, beim Schließen in Menüleistenmodus zurückkehren.

5. A-013: Developer-ID-Identität und Notarytool-Profil prüfen; bei vorhandenem Zugang lokales Bundle signieren/notarisieren/stapeln und Gatekeeper prüfen. Fehlende Apple-Voraussetzungen als konkreten Blocker melden. Nutzer hat diesen lokalen Apple-Freigabeschritt ausdrücklich autorisiert.

Aktueller Zielabgleich (2026-10-29T08:32:58+01:00): 33 Tests grün, Ego Lite abgeschlossen, native Öffnen-/Schließen-Policy geprüft, finaler signierter App-/DMG-Build und Signaturen geprüft. Apple-Notarisierung wartet auf einmalige lokale Zugangseinrichtung durch Claus. Kürzester nächster Schritt: Schlüsselbundprofil lotse einrichten und scripts/notarize.sh lotse ausführen; danach Ticket und Gatekeeper-Freigabe dokumentieren.

Status: A-009/A-011/A-012 geprüft, A-010 technisch geprüft (Geschmacksabnahme offen); A-013 signiert, Notarisierung mangels Apple-Zugangsprofil blockiert. Annahme: Zeitangaben für eine konkrete Übung bekommen Timer; vergangene Arbeitszeit und „in 10 min erinnern“ sind keine Übungsdauer.

---

**Sichtbares Arbeitsziel:** Claus kann Lotse lokal als Mac-App nutzen; die Kopfzeile zeigt das originale KI-AffAIrs-Logo mit der Produktkennung Lotse.
**Zielort:** `target/release/bundle/macos/Lotse.app` im lokalen Projektordner.
**Abschlussnachweis:** Core-Test-Suite und Produktions-Build erfolgreich; App-Bundle am Zielort vorhanden.

Stand: 2026-10-28

Zweck: [ZIEL.md](ZIEL.md) · Vollständige Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

## Historischer Umfang (2026-10-28)

- In diesem Arbeitslauf: A-005 bis A-008 (A-001 bis A-004 sind geprüft)
- Bewusst außerhalb dieses Laufs: signierter/notarisierter Release und Installation nach `/Applications`; nicht beauftragt.
- Wesentliche Annahmen: Keine zur Rhythmusart; der Nutzer hat Aktivitätsbasis verbindlich gewählt. Der macOS-Systemton für A-003 bleibt standardmäßig aus.
- Anwendbare Learnings: Gemeinsame L-001 (Zielort nachweisen) und L-002 (jede Anforderung einzeln abgleichen). Prüfpunkte: App-Bundle tatsächlich prüfen und alle drei IDs mit Belegen abschließen.

## Arbeitsschritte

| Schritt | Anforderungs-IDs | Konkretes Ergebnis | Abhängigkeit | Status | Prüfung |
| --- | --- | --- | --- | --- | --- |
| 1 | A-001–A-003 | Repository und vorhandene Reminder-Architektur erfassen; Projektführung anlegen. | Clone vorhanden | erledigt | Quell- und Dokumentprüfung am 2026-10-28 |
| 2 | A-003 | Persistenten Schalter und sichere macOS-Systemton-Ausgabe an die vorhandene Hinweis-Ausgabe anbinden. | Schritt 1 | erledigt | Frontend- und Produktions-Build erfolgreich, 2026-10-28 |
| 3 | A-001–A-003 | Regressionen und lokalen Produktions-Build ausführen, App-Bundle prüfen. | Schritt 2; Node-Abhängigkeiten | erledigt | `cargo test -p lotse-core` (25/25), `npm run tauri build` erfolgreich, App + DMG geprüft |
| 4 | A-001–A-003 | Dokumentation, Abgleich, lokaler Commit und globales Arbeitslogbuch aktualisieren. | Schritt 3 | erledigt | Lokaler Commit `fbc544a`; globaler Worklog-Eintrag folgt separat |
| 5 | A-004 | Aktivitätsbasiertes Verhalten mit einem fokussierten Regressionstest festhalten und dokumentieren. | Nutzerentscheidung | erledigt | `inactive_time_does_not_start_the_reminder_rhythm`; Core-Tests 26/26 |
| 6 | A-005 | Reproduzierenden Test für die blockierte Prozessliste erstellen. | Befund: 220883 Byte `ps`-Ausgabe | erledigt | Test war rot mit `Zeitüberschreitung` und nach Fix grün |
| 7 | A-005 | Ausgabe von Kindprozessen während der Laufzeit sicher drainen und Codex-Komponenten getrennt zuordnen. | Schritt 6 | erledigt | Großausgabe- und eingebetteter-Codex-Test grün |
| 8 | A-005 | Lokale App neu bauen, Cockpit mit realen 8 Tabs sowie Claude/Codex prüfen und dokumentieren. | Schritt 7 | in Arbeit | Produktions-Build erfolgreich; neue Instanz läuft, Sichtprüfung offen |
| 9 | A-006 | „Swopper“ im Haltungsmodell, Cockpit, Menü und Hinweisen durch „Bürostuhl“ ersetzen; bestehende Sitz-ID für gespeicherte Zustände beibehalten. | Nutzerkorrektur | erledigt | Technische Kennung unverändert; sichtbare Bezeichnung konsistent ersetzt |
| 10 | A-006 | Standard und Wechselhinweis per Core-Test prüfen. | Schritt 9 | erledigt | `office_chair_is_the_default_and_prompts_a_posture_change` grün |
| 11 | A-006 | Lokales Bundle bauen und die sichtbare Standardhaltung im Cockpit prüfen. | Schritte 9–10 | erledigt | Produktions-Build erstellt; laufendes Cockpit zeigt Bürostuhl als Standardhaltung |
| 12 | A-007 | Starres Drei-Haltungsmodell in eine lokal gespeicherte Haltungs-Liste mit vier Standards überführen; alte lokale Zustände migrieren. | Nutzerkorrektur | erledigt | `sitting` wird als Bürostuhl migriert; alte Intervalle werden übernommen |
| 13 | A-007 | Cockpit und Wechselhinweise aus der gespeicherten Haltungs-Liste erzeugen. | Schritt 12 | erledigt | Auswahl und Hint-Aktionen entstehen dynamisch aus der Liste |
| 14 | A-007 | Im Einstellungsbereich Hinzufügen, Intervalldauer und Entfernen implementieren. | Schritt 12 | erledigt | lokale Liste mit Mindestanzahl eins umgesetzt |
| 15 | A-007 | Core-Tests, lokales Bundle und sichtbare Cockpit-Prüfung durchführen. | Schritte 12–14 | erledigt | 30 Tests grün; Tauri-Bundle und DMG erstellt; laufende Einstellungen zeigen vier Haltungen, Hinzufügen und Entfernen |
| 16 | A-008 | Vorhandenes Original-Logo als passende lokale Quelle prüfen und in die Kopfzeile einbinden. | Nutzerwunsch | erledigt | KI-AffAIrs-Banner enthält das Original-Logo |
| 17 | A-008 | Kopfzeilen-Layout mit kompakter Logo-Marke und „Lotse“ gestalten. | Schritt 16 | erledigt | Original-Bildmarke als dezenter lokaler Ausschnitt eingebunden |
| 18 | A-008 | Lokales Tauri-Bundle erstellen und Kopfzeile sichtbar prüfen. | Schritt 17 | erledigt | Produktions-Bundle und DMG erstellt; laufendes Cockpit zeigt die Original-Bildmarke neben „Lotse“ |

## Blockaden und Entscheidungen

- A-004: Aktivitätsbasiert ist entschieden. Der Tageszustand bleibt bei Neustarts erhalten; ein später gewünschter Neustart-Reset wäre eine neue Anforderung.
- A-005: Die Prozessausgabe übersteigt den Pipe-Puffer. Der Fehler liegt vor der Ghostty- und Desktop-App-Zuordnung; die Erfassung muss zuerst zuverlässig fertiglesen.
- A-006: Kein neues Haltungsformat nötig: Die persistierte technische Kennung `sitting` bleibt aus Kompatibilitätsgründen bestehen und wird überall als „Bürostuhl“ gezeigt.
- A-007: Das starre Haltungsformat reicht für persönlich verwaltbare Haltungen nicht aus. Bestehende Sitzdaten werden beim Laden als Bürostuhl übernommen, damit nichts verlorengeht.
- A-008: Es gibt kein separates Logo-Asset. Der vorhandene, originale Markenbanner liefert den Logo-Ausschnitt; kein generiertes oder nachgezeichnetes Zeichen wird verwendet.
- Ein echter Klangtest mit hörbarer Ausgabe ist nicht automatisierbar; die direkte macOS-Ausführung und der Build sind der technische Nachweis, die Wahl bleibt über den Schalter kontrollierbar.
- A-001: Xcode 27 erzeugte ohne `-Wl,-no_fixup_chains` nicht ladbare Rust-Makro-Dylibs. Die macOS-spezifische Cargo-Konfiguration enthält den verifizierten Build-Workaround.

## Historischer Zielabgleich (2026-10-28)

- Arbeitsbeginn / letzter Abgleich: 2026-10-28 14:05 CET / 2026-10-28 17:09 CET
- Nächster Abgleich spätestens: bei der nächsten gezielten Erkennungsprüfung.
- Zum sichtbaren Ergebnis fehlt: A-005 unter der ursprünglichen konkreten Arbeitslage erneut bestätigen (8 Ghostty-Tabs sowie Claude Desktop und Codex App getrennt).
- Laufender Schritt notwendig wegen: nur A-005; A-006 bis A-008 sind im laufenden Cockpit sichtbar bestätigt.
- Kürzester zulässiger nächster Schritt: Bei erneut vorliegender 8-Tab-Arbeitslage Cockpit und Aktivitätsliste gezielt prüfen; private Tabtitel nicht sichern oder veröffentlichen.
- Weiter gültige Prüfbelege: `cargo test --workspace` (30/30) und `npm run tauri build` (App und DMG), jeweils 2026-10-28; Sichtprüfung des Cockpits und der Einstellungen 2026-10-28.

## Nächster konkreter Schritt

Bei erneut vorliegender 8-Tab-Arbeitslage A-005 sichtbar bestätigen; A-006 bis A-008 nicht erneut öffnen, sofern sich die Implementierung nicht ändert.

## Abschlussprüfung

- [x] Sichtbares Ergebnis am vereinbarten Zielort nachgewiesen.
- [x] Alle ursprünglichen und nachträglichen Wünsche mit AENDERUNGEN.md abgeglichen.
- [x] Alle IDs haben einen belegten Status.
- [x] Passende Tests und Prüfungen frisch ausgeführt.
- [x] Arbeitslogbuch und Versionszuordnung aktualisiert.
- [x] Neue Nutzerkorrekturen/Learnings erfasst oder ausdrücklich nicht angefallen.
