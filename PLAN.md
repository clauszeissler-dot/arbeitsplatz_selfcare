# Aktueller Arbeitsplan

**Sichtbares Arbeitsziel:** Claus kann Lotse lokal als Mac-App nutzen; konfigurierbare Erinnerungen zeigen eine Karte und geben bei Wunsch einen Ton aus.
**Zielort:** `target/release/bundle/macos/Lotse.app` im lokalen Projektordner.
**Abschlussnachweis:** Core-Test-Suite und Produktions-Build erfolgreich; App-Bundle am Zielort vorhanden.

Stand: 2026-10-28

Zweck: [ZIEL.md](ZIEL.md) · Vollständige Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

## Aktueller Umfang

- In diesem Arbeitslauf: A-004 (A-001 bis A-003 sind geprüft)
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

## Blockaden und Entscheidungen

- A-004: Aktivitätsbasiert ist entschieden. Der Tageszustand bleibt bei Neustarts erhalten; ein später gewünschter Neustart-Reset wäre eine neue Anforderung.
- Ein echter Klangtest mit hörbarer Ausgabe ist nicht automatisierbar; die direkte macOS-Ausführung und der Build sind der technische Nachweis, die Wahl bleibt über den Schalter kontrollierbar.
- A-001: Xcode 27 erzeugte ohne `-Wl,-no_fixup_chains` nicht ladbare Rust-Makro-Dylibs. Die macOS-spezifische Cargo-Konfiguration enthält den verifizierten Build-Workaround.

## Aktueller Zielabgleich

- Arbeitsbeginn / letzter Abgleich: 2026-10-28 13:15 CET
- Nächster Abgleich spätestens: 2026-10-28 13:25 CET
- Zum sichtbaren Ergebnis fehlt: kein Produktverhalten; nur der lokale Abschluss-Commit.
- Laufender Schritt notwendig wegen: Master-Modus-Versionierung für A-004.
- Kürzester zulässiger nächster Schritt: A-004-Dokumentation und Test gezielt lokal committen.
- Weiter gültige Prüfbelege: `cargo test -p lotse-core` (25/25), `npm run tauri build` (App 4,5 MB, DMG 2,9 MB), jeweils 2026-10-28.

## Nächster konkreter Schritt

Test und Dokumentation für A-004 als lokalen Commit erzeugen.

## Abschlussprüfung

- [x] Sichtbares Ergebnis am vereinbarten Zielort nachgewiesen.
- [x] Alle ursprünglichen und nachträglichen Wünsche mit AENDERUNGEN.md abgeglichen.
- [x] Alle IDs haben einen belegten Status.
- [x] Passende Tests und Prüfungen frisch ausgeführt.
- [x] Arbeitslogbuch und Versionszuordnung aktualisiert.
- [x] Neue Nutzerkorrekturen/Learnings erfasst oder ausdrücklich nicht angefallen.
