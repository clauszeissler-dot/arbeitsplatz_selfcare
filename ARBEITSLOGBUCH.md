# Projekt-Arbeitslogbuch

Ziel: [ZIEL.md](ZIEL.md) · Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Aktueller Einstieg: [PLAN.md](PLAN.md)

## 2026-10-28 – Lokale Projektaufnahme und Audio-Anforderung

- Anforderungs-IDs: A-001 bis A-003
- Tatsächliche Änderungen und Ergebnis: Repository nach `arbeitsplatz_selfcare` geklont und die verbindlichen Projektdateien angelegt. Bestehendes MVP geprüft: lokaler Tauri-Menüleistenbetrieb, konfigurierbare Pausen-/Trink-/Haltungsintervalle, visuelle Toast-Karte und optionaler Anmeldestart sind bereits vorhanden.
- Wesentliche Entscheidung und Begründung / Quelle: Der bestehende konfigurierbare Rhythmus bleibt erhalten. Ergänzt wird nur der explizit gewünschte, abschaltbare Ton als Systemton; kein eigener Audio-Stack nötig.
- Ergebnis am vereinbarten Zielort: noch ausstehend.
- Prüfung: Quellprüfung von `core/src/engine.rs`, `core/src/settings.rs`, `src-tauri/src/lib.rs`, Dashboard und Anleitung am 2026-10-28.
- Red / Green: Nicht anwendbar; Erweiterung eines bestehenden Produktverhaltens, kein Reproduktionsfehler.
- Offene Punkte und nächster Schritt: A-003 implementieren, anschließend Core-Tests und Tauri-Build durchführen.
- Versionsbezug: noch nicht committed

## 2026-10-28 – Bürostuhl als Standardhaltung und Wechselhinweis

- Anforderungs-ID: A-006
- Tatsächliche Änderungen und Ergebnis: Der bestehende technische Sitz-Zustand bleibt zur Kompatibilität mit gespeicherten lokalen Daten erhalten, erscheint aber im Cockpit, Menü, Einstellungen und allen Hinweisen nun als „Bürostuhl“. Der Ablauf nach dem Sitzintervall heißt ausdrücklich „Haltung wechseln“ und bietet Stehen oder Board an.
- Wesentliche Entscheidung und Begründung / Quelle: Der normale Bürostuhl ist laut Nutzer die tatsächliche Ausgangshaltung. Deshalb wurde kein vierter, inkompatibler Speicherzustand eingeführt, sondern der vorhandene Standard verständlich bezeichnet.
- Red / Green: Der neue fokussierte Test war rot, weil die Anwendung „Swopper“ zurückgab. Nach der Umbenennung ist `office_chair_is_the_default_and_prompts_a_posture_change` grün und prüft Standard, Intervall, Wechseltext sowie beide Aktionen.
- Prüfung: `cargo test --workspace` – 29 Tests bestanden. `npm run tauri build` – lokales `Lotse.app` und DMG erzeugt; ausführbares App-Binary und DMG danach vorhanden. Die gezielt neu gestartete Projekt-App läuft aus `target/release/bundle/macos/`.
- Ergebnis am vereinbarten Zielort: Das neue Bundle steht bereit und läuft; die Bildschirmprüfung ist offen, weil die Menüleisten-App ihr Cockpit absichtlich versteckt startet und die zugelassene UI-Abfrage darin einen Zeitüberschreitungsfehler meldete.
- Offene Punkte und nächster Schritt: Im Lotse-Menü „Cockpit öffnen“ und die sichtbare Standardhaltung „Bürostuhl“ bestätigen; parallel die schon offene A-005-Cockpit-Prüfung abschließen.
- Versionsbezug: noch nicht committed

## 2026-10-28 – Prozessauslese und Codex-Zuordnung repariert

- Anforderungs-IDs: A-005
- Tatsächliche Änderungen und Ergebnis: `collect::run` leert Standard- und Fehlerausgabe nun parallel zur Prozesswartezeit. Dadurch blockieren große `ps`-Ausgaben nicht mehr am Pipe-Puffer. Eingebettete Codex-Komponenten unter `ChatGPT.app` werden vor der allgemeinen ChatGPT-Regel als „Codex App“ klassifiziert.
- Red / Green: Der Großausgabe-Test schlug vorher reproduzierbar mit `Zeitüberschreitung` fehl und ist nach der Änderung grün. Der Codex-Zuordnungstest war vorher „ChatGPT“ statt „Codex App“ und ist grün.
- Prüfung: `cargo test --workspace` – 28 Tests bestanden. `npm run tauri build` – Produktions-Bundle erfolgreich erstellt. Die vorherige Projekt-App-Instanz wurde gezielt durch das aktuelle Bundle ersetzt und neu gestartet.
- Ergebnis am vereinbarten Zielort: Neue `Lotse.app` läuft aus `target/release/bundle/macos/`; die sichtbare Cockpit-Abnahme bleibt offen.
- Offene Punkte und nächster Schritt: Im geöffneten Cockpit die erwarteten 8 Tabs sowie Claude Desktop und Codex App prüfen.
- Versionsbezug: noch nicht committed

## 2026-10-28 – Erkennungsfehler: Ghostty und Desktop-Apps

- Anforderungs-IDs: A-005
- Tatsächliche Beobachtung: Das laufende Cockpit zeigt „Ghostty läuft gerade nicht“, 0 Tabs und 0 KI-Agenten. Die direkte Ghostty-AppleScript-Abfrage liefert hingegen 8 Tabs; Claude- und ChatGPT-Codex-Prozesse laufen lokal.
- Belegte Ursache: `ps` erzeugt 220883 Byte Ausgabe. `collect::run` wartet vor dem Lesen der umgeleiteten Ausgabe, sodass der Kindprozess am Pipe-Puffer blockiert, nach 5 Sekunden abgebrochen wird und eine leere Prozessliste entsteht.
- Nächster Schritt: roten Regressionstest ausführen, danach die Prozessausgabe beim Lauf lesen.
- Versionsbezug: noch nicht committed
- Learning-Bezug: Gemeinsame L-001 und L-002 angewendet; Wirksamkeitsbeleg folgt mit Zielortprüfung.

## 2026-10-28 – Audio-Option, lokaler Build und Abnahme

- Anforderungs-IDs: A-001 bis A-003
- Tatsächliche Änderungen und Ergebnis: `sound_enabled` als lokale, standardmäßig ausgeschaltete Einstellung ergänzt; Dashboard-Schalter speichert die Wahl. Bei jeder tatsächlich angezeigten Karte startet Lotse bei aktivem Schalter den macOS-Systemton `Glass.aiff`; der Tonfehler kann die Karte nicht verhindern. Die Dokumentation beschreibt die Option.
- Wesentliche Entscheidung und Begründung / Quelle: Der Systemton vermeidet eine zusätzliche Audio-Abhängigkeit, neue Berechtigungen und Netzverkehr. Der Standard ist aus, damit der Ton bewusst aktiviert wird.
- Wesentliche Zielabweichung oder Kurskorrektur: Der Standard-Build scheiterte an Xcode-27-Linker-Dylibs. `.cargo/config.toml` setzt ausschließlich für macOS `-Wl,-no_fixup_chains`; der anschließende reguläre Projekt-Build war erfolgreich.
- Ergebnis am vereinbarten Zielort: `target/release/bundle/macos/Lotse.app` (4,5 MB) und die DMG (2,9 MB) vorhanden; Bundle-Identifier `de.kiaffairs.lotse` geprüft.
- Prüfung: `cargo test -p lotse-core` – 25/25 erfolgreich. `npm run tauri build` – TypeScript/Vite und Tauri-Bundle erfolgreich. `git diff --check` erfolgreich.
- Red / Green: Kein reproduzierter Produktfehler; Regressionstest plus Produktions-Build sind der passende Nachweis.
- Offene Punkte und nächster Schritt: Produktentscheidung zu „ab Start“ bei Bedarf klären (Tageszustand behalten oder bei jedem Start zurücksetzen).
- Versionsbezug: Lokaler Commit `fbc544a` (`feat(A-001-A-003): lokale Lotse-Erinnerungen abrunden`); kein Push.
- Learning-Bezug: Gemeinsame L-001 nachgewiesen: App-Bundle am Zielort geprüft. Keine neue bestätigte projektspezifische Erkenntnis.

## 2026-10-28 – Aktivitätsbasierter Erinnerungsrhythmus bestätigt

- Anforderungs-IDs: A-004
- Tatsächliche Änderungen und Ergebnis: Die Nutzerentscheidung „aktivitätsbasiert“ als verbindliche Produktregel aufgenommen. Ein fokussierter Core-Test stellt sicher, dass reine Inaktivität den Rhythmus nicht startet und die erste Aktivität den Zähler bei null beginnt.
- Wesentliche Entscheidung und Begründung / Quelle: Nutzerentscheidung nach Erläuterung der Varianten.
- Ergebnis am vereinbarten Zielort: Die lokal gebaute App behält ihr aktivitätsbasiertes Verhalten; der Core-Test prüft den Startpunkt des Rhythmus.
- Prüfung: `cargo test -p lotse-core` – 26/26 erfolgreich, darunter `inactive_time_does_not_start_the_reminder_rhythm`.
- Offene Punkte und nächster Schritt: lokalen Commit erzeugen.
- Versionsbezug: noch nicht committed
