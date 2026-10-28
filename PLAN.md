# Aktueller Arbeitsplan

**Sichtbares Arbeitsziel:** Claus kann Lotse lokal als Mac-App nutzen; konfigurierbare Erinnerungen zeigen eine Karte und geben bei Wunsch einen Ton aus.
**Zielort:** `target/release/bundle/macos/Lotse.app` im lokalen Projektordner.
**Abschlussnachweis:** Core-Test-Suite und Produktions-Build erfolgreich; App-Bundle am Zielort vorhanden.

Stand: 2026-10-28

Zweck: [ZIEL.md](ZIEL.md) · Vollständige Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

## Aktueller Umfang

- In diesem Arbeitslauf: A-001, A-002, A-003
- Bewusst außerhalb dieses Laufs: signierter/notarisierter Release und Installation nach `/Applications`; nicht beauftragt.
- Wesentliche Annahmen: Der vorhandene, pro Hinweisart einstellbare und aktivitätsbasierte Rhythmus erfüllt A-002 bis zur Klärung von „ab Start“; ein macOS-Systemton ist für A-003 ausreichend und benötigt weder Netz noch neue Berechtigungen. Der Ton ist standardmäßig aus, damit er bewusst eingeschaltet wird.
- Anwendbare Learnings: Gemeinsame L-001 (Zielort nachweisen) und L-002 (jede Anforderung einzeln abgleichen). Prüfpunkte: App-Bundle tatsächlich prüfen und alle drei IDs mit Belegen abschließen.

## Arbeitsschritte

| Schritt | Anforderungs-IDs | Konkretes Ergebnis | Abhängigkeit | Status | Prüfung |
| --- | --- | --- | --- | --- | --- |
| 1 | A-001–A-003 | Repository und vorhandene Reminder-Architektur erfassen; Projektführung anlegen. | Clone vorhanden | erledigt | Quell- und Dokumentprüfung am 2026-10-28 |
| 2 | A-003 | Persistenten Schalter und sichere macOS-Systemton-Ausgabe an die vorhandene Hinweis-Ausgabe anbinden. | Schritt 1 | erledigt | Frontend- und Produktions-Build erfolgreich, 2026-10-28 |
| 3 | A-001–A-003 | Regressionen und lokalen Produktions-Build ausführen, App-Bundle prüfen. | Schritt 2; Node-Abhängigkeiten | erledigt | `cargo test -p lotse-core` (25/25), `npm run tauri build` erfolgreich, App + DMG geprüft |
| 4 | A-001–A-003 | Dokumentation, Abgleich, lokaler Commit und globales Arbeitslogbuch aktualisieren. | Schritt 3 | erledigt | Lokaler Commit `fbc544a`; globaler Worklog-Eintrag folgt separat |

## Blockaden und Entscheidungen

- A-002: „ab Start“ ist semantisch offen: bestehend beginnt der Rhythmus mit erkannter Aktivität und derselbe Tageszustand überlebt Neustarts. Die bestehende sichere Variante bleibt erhalten; für einen Reset bei jedem Start oder feste Uhrzeiten ist eine Nutzerentscheidung nötig.
- Ein echter Klangtest mit hörbarer Ausgabe ist nicht automatisierbar; die direkte macOS-Ausführung und der Build sind der technische Nachweis, die Wahl bleibt über den Schalter kontrollierbar.
- A-001: Xcode 27 erzeugte ohne `-Wl,-no_fixup_chains` nicht ladbare Rust-Makro-Dylibs. Die macOS-spezifische Cargo-Konfiguration enthält den verifizierten Build-Workaround.

## Aktueller Zielabgleich

- Arbeitsbeginn / letzter Abgleich: 2026-10-28 13:03 CET / 2026-10-28 13:13 CET
- Nächster Abgleich spätestens: nicht erforderlich; Produktziel erreicht.
- Zum sichtbaren Ergebnis fehlt: keine Produktänderung. Der globale Worklog-Eintrag wird außerhalb dieses Repositorys geführt.
- Laufender Schritt notwendig wegen: Arbeitslogbuch-Pflicht für A-001 bis A-003.
- Kürzester zulässiger nächster Schritt: globalen Worklog-Eintrag anlegen.
- Weiter gültige Prüfbelege: `cargo test -p lotse-core` (25/25), `npm run tauri build` (App 4,5 MB, DMG 2,9 MB), jeweils 2026-10-28.

## Nächster konkreter Schritt

Keine weitere Projektänderung; der globale Worklog-Eintrag wird anschließend angelegt.

## Abschlussprüfung

- [x] Sichtbares Ergebnis am vereinbarten Zielort nachgewiesen.
- [x] Alle ursprünglichen und nachträglichen Wünsche mit AENDERUNGEN.md abgeglichen.
- [x] Alle IDs haben einen belegten Status.
- [x] Passende Tests und Prüfungen frisch ausgeführt.
- [x] Arbeitslogbuch und Versionszuordnung aktualisiert.
- [x] Neue Nutzerkorrekturen/Learnings erfasst oder ausdrücklich nicht angefallen.
