# Aktueller Arbeitsplan

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
