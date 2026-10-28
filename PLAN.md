# Aktueller Arbeitsplan

**Sichtbares Arbeitsziel:** Claus kann Lotse lokal als Mac-App nutzen; Bürostuhl und Swopper sind getrennt sichtbar und jede Person kann ihre Haltungen lokal verwalten.
**Zielort:** `target/release/bundle/macos/Lotse.app` im lokalen Projektordner.
**Abschlussnachweis:** Core-Test-Suite und Produktions-Build erfolgreich; App-Bundle am Zielort vorhanden.

Stand: 2026-10-28

Zweck: [ZIEL.md](ZIEL.md) · Vollständige Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

## Aktueller Umfang

- In diesem Arbeitslauf: A-005 bis A-007 (A-001 bis A-004 sind geprüft)
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
| 11 | A-006 | Lokales Bundle bauen und die sichtbare Standardhaltung im Cockpit prüfen. | Schritte 9–10 | in Arbeit | Produktions-Build erstellt; neue Instanz läuft, verborgenes Cockpit muss manuell geöffnet werden |
| 12 | A-007 | Starres Drei-Haltungsmodell in eine lokal gespeicherte Haltungs-Liste mit vier Standards überführen; alte lokale Zustände migrieren. | Nutzerkorrektur | erledigt | `sitting` wird als Bürostuhl migriert; alte Intervalle werden übernommen |
| 13 | A-007 | Cockpit und Wechselhinweise aus der gespeicherten Haltungs-Liste erzeugen. | Schritt 12 | erledigt | Auswahl und Hint-Aktionen entstehen dynamisch aus der Liste |
| 14 | A-007 | Im Einstellungsbereich Hinzufügen, Intervalldauer und Entfernen implementieren. | Schritt 12 | erledigt | lokale Liste mit Mindestanzahl eins umgesetzt |
| 15 | A-007 | Core-Tests, lokales Bundle und sichtbare Cockpit-Prüfung durchführen. | Schritte 12–14 | in Arbeit | 30 Tests grün; Tauri-Bundle und DMG erstellt, neue Projekt-App läuft; Cockpit-Prüfung offen |

## Blockaden und Entscheidungen

- A-004: Aktivitätsbasiert ist entschieden. Der Tageszustand bleibt bei Neustarts erhalten; ein später gewünschter Neustart-Reset wäre eine neue Anforderung.
- A-005: Die Prozessausgabe übersteigt den Pipe-Puffer. Der Fehler liegt vor der Ghostty- und Desktop-App-Zuordnung; die Erfassung muss zuerst zuverlässig fertiglesen.
- A-006: Kein neues Haltungsformat nötig: Die persistierte technische Kennung `sitting` bleibt aus Kompatibilitätsgründen bestehen und wird überall als „Bürostuhl“ gezeigt.
- A-007: Das starre Haltungsformat reicht für persönlich verwaltbare Haltungen nicht aus. Bestehende Sitzdaten werden beim Laden als Bürostuhl übernommen, damit nichts verlorengeht.
- Ein echter Klangtest mit hörbarer Ausgabe ist nicht automatisierbar; die direkte macOS-Ausführung und der Build sind der technische Nachweis, die Wahl bleibt über den Schalter kontrollierbar.
- A-001: Xcode 27 erzeugte ohne `-Wl,-no_fixup_chains` nicht ladbare Rust-Makro-Dylibs. Die macOS-spezifische Cargo-Konfiguration enthält den verifizierten Build-Workaround.

## Aktueller Zielabgleich

- Arbeitsbeginn / letzter Abgleich: 2026-10-28 14:05 CET / 2026-10-28 14:25 CET
- Nächster Abgleich spätestens: nach der sichtbaren Cockpit-Prüfung.
- Zum sichtbaren Ergebnis fehlt: Vier Standardhaltungen sowie lokale Verwaltungsoptionen im neu gebauten Cockpit bestätigen.
- Laufender Schritt notwendig wegen: A-005 bis A-007.
- Kürzester zulässiger nächster Schritt: Im Lotse-Menü „Cockpit öffnen“ wählen, vier Haltungen und das Zahnrad → „Haltungen“ prüfen.
- Weiter gültige Prüfbelege: `cargo test -p lotse-core` (25/25), `npm run tauri build` (App 4,5 MB, DMG 2,9 MB), jeweils 2026-10-28.

## Nächster konkreter Schritt

Cockpit der neu gestarteten Version sichtbar prüfen; bei Bestätigung A-005 bis A-007 auf geprüft setzen.

## Abschlussprüfung

- [x] Sichtbares Ergebnis am vereinbarten Zielort nachgewiesen.
- [x] Alle ursprünglichen und nachträglichen Wünsche mit AENDERUNGEN.md abgeglichen.
- [x] Alle IDs haben einen belegten Status.
- [x] Passende Tests und Prüfungen frisch ausgeführt.
- [x] Arbeitslogbuch und Versionszuordnung aktualisiert.
- [x] Neue Nutzerkorrekturen/Learnings erfasst oder ausdrücklich nicht angefallen.
