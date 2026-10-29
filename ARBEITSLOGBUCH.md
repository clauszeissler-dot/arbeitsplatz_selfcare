# Projekt-Arbeitslogbuch

Ziel: [ZIEL.md](ZIEL.md) · Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Aktueller Einstieg: [PLAN.md](PLAN.md)

## 2026-10-28 – Sichtprüfung und Tagesübergabe

- Anforderungs-IDs: A-006 bis A-008 geprüft; A-005 bleibt offen.
- Ergebnis: Das laufende Cockpit zeigt das originale KI-AffAIrs-Logo neben „Lotse“ und Bürostuhl als Standardhaltung. In den geöffneten Einstellungen sind Bürostuhl, Swopper, Stehen und Balance Board samt Hinzufügen/Entfernen sichtbar.
- Abgrenzung: Die konkrete Erkennungsabnahme für die zuvor beobachteten acht Ghostty-Tabs und beide Desktop-Apps ist nicht wiederholt worden; daher A-005 nicht als geprüft markieren. Rohansichten mit lokalen Tabtiteln oder Pfaden werden nicht als Social-Media-Asset verwendet.
- Social-Media-Übergabe: Drei datensparsam geprüfte Live-Screenshots liegen im separaten KW45-KI-AffAIrs-Paket; keine Veröffentlichung und keine Terminierung.
- Nachweis: [Handoff](docs/handoffs/20261028_01_tagesabschluss-lotse.md) · [PersonalOS-Aktivität](</Users/kiaffairs/Desktop/personalOS/daily/2026/2026-10-28/activity/01a18345-c858-75d3-9e8d-8a64ba555b4c.md>).


## 2026-10-28 – Lokale Projektaufnahme und Audio-Anforderung

- Anforderungs-IDs: A-001 bis A-003
- Tatsächliche Änderungen und Ergebnis: Repository nach `arbeitsplatz_selfcare` geklont und die verbindlichen Projektdateien angelegt. Bestehendes MVP geprüft: lokaler Tauri-Menüleistenbetrieb, konfigurierbare Pausen-/Trink-/Haltungsintervalle, visuelle Toast-Karte und optionaler Anmeldestart sind bereits vorhanden.
- Wesentliche Entscheidung und Begründung / Quelle: Der bestehende konfigurierbare Rhythmus bleibt erhalten. Ergänzt wird nur der explizit gewünschte, abschaltbare Ton als Systemton; kein eigener Audio-Stack nötig.
- Ergebnis am vereinbarten Zielort: noch ausstehend.
- Prüfung: Quellprüfung von `core/src/engine.rs`, `core/src/settings.rs`, `src-tauri/src/lib.rs`, Dashboard und Anleitung am 2026-10-28.
- Red / Green: Nicht anwendbar; Erweiterung eines bestehenden Produktverhaltens, kein Reproduktionsfehler.
- Offene Punkte und nächster Schritt: A-003 implementieren, anschließend Core-Tests und Tauri-Build durchführen.
- Versionsbezug: noch nicht committed

## 2026-10-28 – Originale KI-AffAIrs-Marke in der Kopfzeile

- Anforderungs-ID: A-008
- Tatsächliche Änderungen und Ergebnis: Der Textzug „KI AffAIrs“ wurde durch eine kompakte Bildmarke aus dem vorhandenen, originalen Markenbanner ersetzt; „Lotse“ bleibt als Produktkennung daneben. Es wurde keine neue oder generierte Logo-Variante erstellt.
- Wesentliche Entscheidung und Begründung / Quelle: Das Repository enthält kein separates Logo, aber das Original im bestehenden Markenbanner. Der lokale CSS-Ausschnitt verwendet genau diese Quelle und vermeidet doppelte Markenassets.
- Prüfung: `npm run build` – TypeScript-Prüfung und Frontend-Build erfolgreich. `npm run tauri build` – Produktions-Bundle und DMG erstellt. Die Projekt-App wurde nach einem kurzen macOS-Beendigungsübergang gezielt neu gestartet.
- Ergebnis am vereinbarten Zielort: Neue `Lotse.app` läuft aus `target/release/bundle/macos/`.
- Offene Punkte und nächster Schritt: Im geöffneten Cockpit die kleine Bildmarke neben „Lotse“ sichtbar bestätigen.
- Versionsbezug: Lokaler Commit `91f1ccc` (`feat(A-008): Markenlogo im Cockpit`); kein Push.

## 2026-10-28 – Individuell verwaltbare Haltungen

- Anforderungs-ID: A-007
- Tatsächliche Änderungen und Ergebnis: Das starre Haltungs-Enum wurde durch eine lokale Liste mit Name, stabiler Kennung und Wechselintervall ersetzt. Standard sind Bürostuhl, Swopper, Stehen und Balance Board. Das Cockpit zeigt alle gespeicherten Haltungen; in Einstellungen können Personen Haltungen ergänzen, umbenennen, ihr Intervall festlegen und entfernen. Mindestens eine Haltung bleibt erhalten.
- Kompatibilität: Alte lokale `sitting`-Einträge und deren Tageswerte werden beim Laden in `office_chair` (Bürostuhl) überführt. Ältere Einstellungen ohne Liste erzeugen die vier Standards aus ihren bestehenden Intervallen.
- Red / Green: Der neue Test war rot, da weder eine Haltungs-Liste noch eine eigene Haltung existierten. Er ist grün und prüft Swopper neben Bürostuhl, Auswahl einer persönlichen Haltung, dynamische Wechselaktionen und das Entfernen der gerade aktiven Haltung.
- Prüfung: `cargo test --workspace` – 30 Tests bestanden. `npm run tauri build` – Tauri-Produktions-Bundle und DMG erfolgreich erstellt.
- Ergebnis am vereinbarten Zielort: Das neue Bundle unter `target/release/bundle/macos/Lotse.app` und die DMG sind vorhanden; die gezielt neu gestartete Projekt-App läuft aus dem Bundle.
- Offene Punkte und nächster Schritt: Im Cockpit vier Standardhaltungen sowie Zahnrad → „Haltungen“ mit Hinzufügen/Entfernen sichtbar prüfen.
- Versionsbezug: Lokaler Commit `8d4e37a` (`feat(A-007): Haltungen lokal verwalten`); kein Push.

## 2026-10-28 – Bürostuhl als Standardhaltung und Wechselhinweis

- Anforderungs-ID: A-006
- Tatsächliche Änderungen und Ergebnis: Der bestehende technische Sitz-Zustand bleibt zur Kompatibilität mit gespeicherten lokalen Daten erhalten, erscheint aber im Cockpit, Menü, Einstellungen und allen Hinweisen nun als „Bürostuhl“. Der Ablauf nach dem Sitzintervall heißt ausdrücklich „Haltung wechseln“ und bietet Stehen oder Board an.
- Wesentliche Entscheidung und Begründung / Quelle: Der normale Bürostuhl ist laut Nutzer die tatsächliche Ausgangshaltung. Deshalb wurde kein vierter, inkompatibler Speicherzustand eingeführt, sondern der vorhandene Standard verständlich bezeichnet.
- Red / Green: Der neue fokussierte Test war rot, weil die Anwendung „Swopper“ zurückgab. Nach der Umbenennung ist `office_chair_is_the_default_and_prompts_a_posture_change` grün und prüft Standard, Intervall, Wechseltext sowie beide Aktionen.
- Prüfung: `cargo test --workspace` – 29 Tests bestanden. `npm run tauri build` – lokales `Lotse.app` und DMG erzeugt; ausführbares App-Binary und DMG danach vorhanden. Die gezielt neu gestartete Projekt-App läuft aus `target/release/bundle/macos/`.
- Ergebnis am vereinbarten Zielort: Das neue Bundle steht bereit und läuft; die Bildschirmprüfung ist offen, weil die Menüleisten-App ihr Cockpit absichtlich versteckt startet und die zugelassene UI-Abfrage darin einen Zeitüberschreitungsfehler meldete.
- Offene Punkte und nächster Schritt: Im Lotse-Menü „Cockpit öffnen“ und die sichtbare Standardhaltung „Bürostuhl“ bestätigen; parallel die schon offene A-005-Cockpit-Prüfung abschließen.
- Versionsbezug: Lokaler Commit `d3489ba` (`feat(A-006): Bürostuhl als Standardhaltung`); kein Push.

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


## 2026-10-29 – Laufender Auftrag: Timer, Klanggestaltung und Dashboard-Erreichbarkeit

- A-009: Nutzer möchte insbesondere den 20-Sekunden-Augenhinweis starten können, mit Countdown und hörbarem Abschluss. Konkrete Übungsdauer wird strukturiert geführt; vergangene Arbeitszeit und Verschieben sind keine Timer.
- A-010: Bestehender Klang ist tatsächlich macOS Glass. Eigene lokale Stereo-Klänge für Hinweis, Start und Abschluss werden erstellt; keine Cloud-Abhängigkeit. Klanggeschmack wird nicht als technisch bewiesen ausgegeben.
- A-011: Nutzer verlangt einen laufenden Arbeitslog, damit Ergänzungen erhalten bleiben. Dieser Eintrag sowie PLAN.md/AENDERUNGEN.md führen den aktuellen Stand.
- A-012: Nutzer findet das geöffnete Dashboard nicht im Dock/⌘Tab. Befund: dauerhaft `ActivationPolicy::Accessory`. Öffnen soll auf Regular umschalten, Schließen wieder auf Accessory; Tray bleibt erhalten.
- Prüfstand: Umsetzung läuft; erster roter Regressionstest für strukturierte Übungsdauer gestartet. Lokales App-Bundle ist Ziel; kein Push/Release/Installation beauftragt.

- Ergänzung A-012: Nutzer bestätigt Menüleiste bei geschlossenem Fenster; bei geöffnetem Fenster Dock/⌘Tab erforderlich.
- Ergänzung A-013: Nutzer autorisiert Apple-Developer-Freigabe. Vorprüfung von Developer ID und lokalem Notarytool-Profil gestartet. Kein zusätzlicher öffentlicher Release/Push abgeleitet.

- Zwischenstand: Regressionstest war rot (fehlende strukturierte 20-s-Dauer), danach grün für Augen- und individuelle Pausendauern. Workspace: 33 Tests grün, inklusive nativer Countdown-Deadline, Aufrunden, Überbrücken von Standby, mehrfacher Start und einmaliger Abschluss.
- Ego Lite, TaskSpace 5: Augenhinweis gestartet und Abschluss bei 0:00/Fertig nachgewiesen. Screenshot zeigte zusätzlichen Platzbedarf; native Fensterhöhe auf 380 angepasst, Kartenbreite auf 364 begrenzt. Laufender Timer wird nicht automatisch ausgeblendet und nicht durch neue Hinweise ersetzt.
- Apple: gültiges Developer-ID-Zertifikat vorhanden; erster Produktions-Build signiert. Profil `lotse` fehlt; Nutzer bestätigt keine bisherige Einreichung. Sichere lokale Einrichtung mit `notarytool store-credentials` beschrieben; keine Zugangsdaten im Chat.


### Abschlussstand 2026-10-29T08:32:58+01:00

- A-009: Strukturierte Dauer für Augen und Pausen; Pausentexte nutzen die eingestellte Dauer. Native Deadline mit 200-ms-Auswertung verhindert Webview-Drosselung; Timer bleibt bis Fertig/Abbruch erhalten, konkurrierende Hinweise werden unterdrückt. Abbrechen/Schließen einer gestarteten Pause beendet auch deren Pausenerfassung.
- A-010: Originale, reproduzierbare Stereo-Kompositionen (D-Dur-Pentatonik, weiche Teiltöne, Stereo-Echos; 1,9/2,8/4,4 s). Drei WAVs lokal eingebettet, kein Glass. Technische Analyse: alle Peaks -12 dBFS, beide Endsamples null. Hinweisschalter bleibt standardmäßig aus; bewusste Timerstarts haben unabhängigen Klangschalter. Einstellungen bieten drei Klangproben. Geschmack durch Claus noch nicht bewertet.
- Red/Green: Dauertest zuerst rot (Null statt 20 s), danach grün für 20 s / 7 min / 22 min. Frisch: `cargo test --workspace`, 33/33; `npm run tauri build` inklusive TypeScript/Vite erfolgreich. `bash -n scripts/notarize.sh` und `git diff --check` grün.
- Ego Lite TaskSpace 5: 20 s im Echtzeitablauf bis 0:00/Fertig; instrumentiert genau ein Start- und ein Abschlussaufruf; stummer 25-min-Start und Abbruch ohne Klangaufruf. Start/Fertig per Enter bedienbar; Status-Live-Region vorhanden. Einstellungen-Klangprobe lädt die 776204-Byte-WAV ohne Fehler; keine unbenannten Buttons im geprüften Einstellungsformular. Fenstergröße 380×380: Pausenkarte 350,94 px hoch, vollständig sichtbar. Screenshot: `docs/assets/timer.png`. TaskSpace danach geschlossen.
- A-012: Finale native Öffnen-/Schließen-Logik über macOS-Reopen und AX geprüft: geöffnet Regular (background-only=false, visible=true), geschlossen Accessory (background-only=true, visible=false). Tray-Menüabfrage über AX war nicht zuverlässig (ungültiger Menüindex); kein Hänger im Prozess-Sample, reguläres Wiederöffnen funktioniert. Menüaktion und Reopen verwenden dieselbe show_dashboard-Funktion. Keine tatsächliche ⌘Tab-Tastensequenz aufgezeichnet.
- A-013: Finales lokales App-Bundle und DMG mit `Developer ID Application: Claus Zeissler (ZWT6395G7Q)` signiert; Hardened Runtime, Zeitstempel und gültige Signatur nachgewiesen. DMG-Integritätsprüfung erfolgreich. Keine Apple-Einreichung erfolgt: Notarytool-Profil lotse fehlt. Gatekeeper weist das Paket derzeit ausdrücklich als Unnotarized Developer ID zurück. Sicherer Einrichtungsweg an Claus gegeben; vollständiger nachfolgender Einreichungs-/Ticket-/Prüfablauf unter `scripts/notarize.sh` vorbereitet, noch nicht gegen Apple ausgeführt.
- A-011: Projektlog, Anforderungen, Plan, Ziel und Learning fortgeschrieben. Globaler Arbeitslogbucheintrag erfolgreich geschrieben; keine persönliche Arbeitszeit angesetzt.
- Versionsbezug: gezielter lokaler Commit folgt; kein Push und keine Veröffentlichung. A-005 aus dem Altauftrag bleibt wie dokumentiert separat offen.
