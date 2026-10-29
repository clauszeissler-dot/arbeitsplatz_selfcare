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


## 2026-10-29T08:44:19+01:00 – Nutzerkorrektur zu vorhandenem Apple-Zugang (A-013)

- Nutzer erinnert an frühere Einrichtung über Apple/Xcode. Gezielte Suche findet den dokumentierten Profilnamen `KI-Kennzeichnung-Notarization` im TimeAffAIrs-Handoff vom 2026-09-22.
- Frischer Nachweis: `notarytool history --keychain-profile KI-Kennzeichnung-Notarization --output-format json` authentifiziert erfolgreich. Keine Passwörter oder Schlüssel ausgelesen; nur Profilreferenz genutzt.
- Fehlerkorrektur: Fehlendes neues Profil lotse belegt keinen fehlenden Apple-Zugang. Aufforderung zur Passwort-Neueinrichtung war unnötig. Früherer Blocker aufgehoben; vorhandener signierter Stand wird jetzt eingereicht.
- Skriptstandard auf das vorhandene Profil geändert. Produktcode und Binärdatei unverändert, vorhandene Tests bleiben gültig.


### A-013 abgeschlossen · 2026-10-29T08:46:44+01:00

- Bestehender Schlüsselbundzugang ohne neues Passwort wiederverwendet. `scripts/notarize.sh` vollständig erfolgreich durchgelaufen.
- App: Apple `Accepted`, Submission `d54284b1-ce4c-41a3-9e02-4f90d0cd471f`. DMG: Apple `Accepted`, Submission `5572a356-ee23-4e09-aa22-54fb11d2d84a`. Beide Apple-Logs: `Ready for distribution`, `issues: null`.
- Original-Bundle: Ticket angebracht, `stapler validate`, `codesign --verify --deep --strict`, `spctl --assess --type execute` erfolgreich, `source=Notarized Developer ID`.
- Versand-ZIP erst nach Stapling neu erzeugt, anschließend in eigenen temporären Ordner entpackt: Signatur, Ticket und Gatekeeper erneut erfolgreich.
- Finales DMG enthält bereits gestapelte App, selbst signiert/notarisiert/gestapelt, Integritätsprüfung grün. Schreibgeschützt ohne Finder-Fenster gemountet: enthaltene App besteht Signatur-, Ticket- und Gatekeeper-Prüfung; eigenen Mount danach gezielt ausgehängt.
- Artefakte: `target/release/notarized/Lotse.zip`, `target/release/notarized/Lotse.dmg`; Apple-Resultate/Logs im selben Ordner. Frühere DMG unter `target/release/bundle/dmg/` ist nur der signierte Zwischenstand; Weitergabe des finalen Pakets aus `notarized/`.
- ZIP SHA-256: `57046b66075c8870c4b617e124203932f280a460b046809854f0bb445af0bde3`.
- DMG SHA-256: `05e13130616461f5d429ddde45b37329d4714ba808954757c838709a5c840475`.
- Anforderungen: A-013 geprüft erfüllt, ursprüngliche Blockerannahme korrigiert. A-009/A-011/A-012 behalten geprüften Stand, A-010 technisch geprüft und Klanggeschmack offen. Altpunkt A-005 bleibt wie bisher getrennt offen.
- Globaler Arbeitslog erfolgreich aktualisiert. Vorhandene Produktprüfungen aus `e5a09a5` gelten unverändert; kein Produktcode geändert, kein zusätzlicher Browserlauf erforderlich. Kein Push, keine öffentliche Veröffentlichung und keine Installation.


## 2026-10-29T09:00:00+01:00 – Repository-Download beauftragt (A-014/A-015)

- Ziel ist das bestehende private GitHub-Repository `clauszeissler-dot/arbeitsplatz_selfcare`, nicht Forgejo. `gh` ist dafür die passende CLI. Kein bestehender Release/Tag; Remote main ist Vorfahr des lokalen geprüften Stands.
- Nutzer autorisiert Upload und README-Link; dazu nötigen Push vorbereiten. Sichtbarkeit bleibt privat. Bestehenden signierten/notarisierten ARM64-Stand unverändert verwenden.
- Tag `lotse-0.1.0` vermeidet Auslösen der separaten `v*`-Universal-Build-Automation. Finale Download-Prüfung anhand SHA-256 vorgesehen.

- Zielabgleich 2026-10-29T09:01:17+01:00: GitHub verweigert Release-Ziel ade59bb, da dieser lokale Commit noch nicht remote existiert. Reihenfolge korrigiert: zuerst autorisierten Stand pushen, dann Release-Entwurf mit Remote-Commit erzeugen. Es wurde kein Release angelegt.
- Verbindliche Clippy-Prüfung vor Push findet `field_reassign_with_default` im bestehenden Migrations-Test. Testinitialisierung wird ohne Änderung am Produktverhalten korrigiert; keine erneute Notarisierung nötig.

- Vor Upload: DMG/ZIP stimmen mit den unter A-013 dokumentierten SHA-256-Werten überein; Architektur `arm64` bestätigt. README und deutsche/englische Anleitung enthalten identischen Direktlink und passende Installationsschritte. Lokale Markdown-Dateilinks geprüft.
- Clippy-Gate nach rein testinterner Initialisierungskorrektur grün, Core-Tests 31/31. Produktcode und signierte Binärdatei unverändert. README/Anleitungen/Release-Dokumentation bereit für autorisierten Push.


### A-014/A-015 abgeschlossen · 2026-10-29T09:04:10+01:00

- Autorisierter Push: alle zuvor lokalen Produktänderungen und Download-Dokumentation als Commit `f1244a5` nach `origin/main` übertragen. Kein fremder Stand überschrieben; Fast-Forward aus dem bisherigen Remote-Stand.
- Release veröffentlicht: https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/tag/lotse-0.1.0, Assets `Lotse.dmg` (5890548 Byte), `Lotse.zip` (5544267 Byte) und `SHA256SUMS` (152 Byte). Repository bleibt privat.
- Abschlussnachweis am Zielort: alle drei Dateien mit `gh release download` in eigenen temporären Ordner erneut von GitHub heruntergeladen; SHA-256 aller Dateien identisch zu den freigegebenen lokalen Quellen. Heruntergeladenes DMG besteht `stapler validate` und `codesign --verify`.
- DMG SHA-256: `05e13130616461f5d429ddde45b37329d4714ba808954757c838709a5c840475`; ZIP SHA-256: `57046b66075c8870c4b617e124203932f280a460b046809854f0bb445af0bde3`.
- Remote-README über GitHub-API auf main gelesen: prominenter Direktlink entspricht exakt der veröffentlichten Asset-URL `/releases/download/lotse-0.1.0/Lotse.dmg`. Beide Installationsanleitungen aktualisiert; Apple-Silicon-Voraussetzung ausdrücklich angegeben.
- Core-Tests 31/31 und Clippy ohne Warnung. GitHub-CI-Kernlogik für `f1244a5` erfolgreich; separater unsignierter Universal-Build zu diesem Zeitpunkt noch in Arbeit. Dieser CI-Build liefert nicht das freigegebene Download-Paket. Vorhandene lokale Produktions-/Notarisierungs-/Ego-Lite-Belege bleiben gültig, da nur Testinitialisierung und Dokumentation geändert wurden.
- Anforderungsabgleich: A-014 und A-015 geprüft erfüllt; keine offenen Punkte für Upload/README. A-005 und subjektive Klangbewertung bleiben wie zuvor getrennt dokumentiert.


## 2026-10-29T09:49:22+01:00 – README auf neue Funktionen abgeglichen (A-016)

- Timer, Sounds, Haltungsverwaltung und notarisierten Download waren vorhanden; Dock/⌘Tab war nur in der Anleitung beschrieben. README ergänzt: Dock/⌘Tab, optionale 20-Sekunden-Augenpause, konfigurierte Pausendauer, drei Klangproben mit getrenntem Timer-Schalter und aktivitätsbasierter Rhythmus.
- Vorhandenen tatsächlichen Ego-Lite-Timer-Screenshot eingebunden. Lokale Inhaltsprüfung gegen Funktionsliste und alle lokalen Markdown-Dateilinks erfolgreich. Binärpakete unverändert; keine neue Signierung/Notarisierung nötig. Remote-Abgleich nach Push.

- A-016 Abschluss: Commit `2cdf97c` nach main übertragen. README anschließend per GitHub-API gelesen und bytegleich mit lokal geprüfter Fassung verglichen. Alle ergänzten Funktionsangaben am Ziel vorhanden. Dokumentation geprüft, keine Produkt-/Paketänderung.


## 2026-10-29T10:06:05+01:00 – Lizenz und Entwickleridentität (A-017/A-018)

- Nutzer erlaubt Nutzung/Weitergabe der Original-App und bestätigt ausdrücklich geänderte Veröffentlichungen unter eigener Identität. Keine Berechtigung zur Verwendung seiner Developer ID oder zum Publizieren veränderter Apps als offizielles Lotse-Produkt.
- Befund: noch keine LICENSE-Datei; Cargo nennt UNLICENSED, npm keine Lizenz. Notarisierungsskript enthält feste Original-Identität und festen Profil-Default. Diese Vorgaben werden für externe Selbstbauer explizit gemacht statt automatisch benutzt.
- Entscheidung: unveränderter Apache-2.0-Text für Code/Doku/eigene Klänge, separate Marken-/Identitätsrichtlinie und README-Zusammenfassung. Offizielles notarisiertes DMG/ZIP nicht verändern; zusätzliche Lizenz-Dateien als Release-Anhänge bereitstellen.

- Implementierung: offizieller Apache-2.0-Text direkt von apache.org übernommen (Ego Lite TaskSpace 16); NOTICE und BRANDING.md erklären Code-Lizenz, Originalweitergabe auch für lotse-0.1.0, Marken-/Grafikausnahmen sowie eigene Identität für Forks. Apache-Abschnitte 4/6 und Apples Code-Signing-Erklärung als Primärquellen geprüft.
- Cargo/npm/Lock-Metadaten auf Apache-2.0 gesetzt; eigene Drittkomponenten werden nicht neu lizenziert. README und beide Selbstbau-Anleitungen verlangen eigene Bundle-ID; Release-/Entwicklungsanleitung erklärt eigene Marken, Signierung und Profile.
- Skript: keine eingebauten Claus-Zertifikats-/Profilvorgaben; Identität und Profil müssen explizit vorliegen. APP_PATH unterstützt eigene App-Namen und Paketdateien. Red/Green: drei Konfigurationstests zuerst rot (externe Aktion trotz fehlender Konfiguration/falscher App-Pfad), nach Anpassung grün. Tests nutzen nur einen lokalen codesign-Stub und stoppen vor realem Schlüsselbund/Apple-Aufruf.
- Vorhandenes DMG/ZIP SHA-256 unverändert zu A-013. Lizenz wird separat an den bestehenden Release gehängt; kein Umverpacken, kein erneutes Signieren oder Notarisieren dieses Downloads.


### A-017/A-018 abgeschlossen · 2026-10-29T10:15:07+01:00

- Commit `1297a97` mit Lizenz, Nutzungs-/Markenrichtlinie, Metadaten, Anleitungen und expliziter Skript-Konfiguration nach GitHub main gepusht.
- Remote-Abnahme: LICENSE, NOTICE, BRANDING.md, README, Cargo.toml und scripts/notarize.sh per GitHub-API bytegleich zur geprüften lokalen Fassung. Alle drei Lizenz-/Hinweis-Dateien beim bestehenden Release angehängt, erneut von GitHub heruntergeladen und identisch bestätigt. Release-Text erklärt Geltung auch für die ursprüngliche Version lotse-0.1.0 und die unveränderten Originalpakete.
- DMG-/ZIP-Digests der veröffentlichten Assets entsprechen weiterhin den unter A-013/A-014 geprüften SHA-256-Werten. Keine Pakete verändert oder erneut signiert.
- Prüfung: offizielle Apache-2.0-Vorlage unverändert; Cargo-Metadaten beider Crates und npm/Lock-Metadaten Apache-2.0; lokale Dateilinks gültig. `bash -n` und drei Konfigurationstests grün. Die Tests werden in die bestehende CI übernommen; kein realer Schlüsselbund- oder Apple-Aufruf im Test. Keine getrackten `.p8`, `.p12`, `.pem` oder `.key`-Dateien gefunden (keine Aussage einer vollständigen Geheimnisprüfung).
- Abgleich: A-017 und A-018 geprüft erfüllt, keine offenen Lizenz-/Dokumentationspunkte dieses Auftrags. Apache-Code-Rechte bleiben unverändert; Marken-/Identitätsabgrenzung wird nicht als Änderung am Standardlizenztext ausgegeben. Frühere Produktprüfungen gelten für den unveränderten Download weiter.
