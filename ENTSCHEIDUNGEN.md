# Entscheidungen und Ergebnisse

Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md) · Umsetzung: [PLAN.md](PLAN.md) · Nachweise: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

AENDERUNGEN.md hält fest, **was** gewünscht war. Diese Datei hält fest, **wie**
es gelöst wurde, wer das entschieden hat und seit wann es live ist.

Angelegt am 2026-11-03 aus AENDERUNGEN.md, Commits und dem Handoff
`docs/handoffs/20261030_01_tagesabschluss-lotse-laufzeit-versionierung.md`.
E-001 bis E-025 (außer E-005) sind aus den Belegen in AENDERUNGEN.md und den
Commits nachgezogen; abgelehnte Alternativen wurden damals nicht festgehalten.
E-005 und E-026 bis E-030 sind ausführlich.

Regeln: E-NNN gehört zu A-NNN; Änderungen als datierter Nachtrag, alter Stand bleibt.

## E-001 – App arbeitet lokal auf dem Mac
- Problem: App arbeitet lokal auf dem Mac.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-001 beschrieben; Abnahmekriterium: Tauri-Produktions-Build erzeugt `Lotse.app`; keine neue Netzwerkfunktion ergänzt.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: fbc544a
- Beleg: `npm run tauri build`, 2026-10-28: App und DMG erstellt
- Offen: –
## E-002 – Erinnerungen folgen einem einstellbaren Rhythmus und geben ein …
- Problem: Erinnerungen folgen einem einstellbaren Rhythmus und geben ein klares visuelles Signal.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-002 beschrieben; Abnahmekriterium: Vorhandene Intervalle und Hinweis-Karte bleiben im Build enthalten; Core-Tests bestehen.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: fbc544a
- Beleg: `cargo test -p lotse-core` (25/25), Produktions-Build erfolgreich
- Offen: –
## E-003 – Akustisches Signal kann genutzt und bewusst ausgeschaltet werden
- Problem: Akustisches Signal kann genutzt und bewusst ausgeschaltet werden.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-003 beschrieben; Abnahmekriterium: Einstellungs-Schalter wird lokal gespeichert; aktiv löst er beim Hinweis einen Systemton aus, inaktiv überspringt er ihn.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: fbc544a
- Beleg: Einstellungen-Serialisierungstest und Produktions-Build erfolgreich; Laufzeit-Ton wird nur bei angezeigter Karte ausgelöst
- Offen: –
## E-004 – Erinnerungsrhythmus arbeitet aktivitätsbasiert
- Problem: Erinnerungsrhythmus arbeitet aktivitätsbasiert.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-004 beschrieben; Abnahmekriterium: Ohne erkannte Eingabe startet keine Reminder-Uhr; erst erkannte Aktivität startet sie. Pausen und Meetings erzeugen keine Hinweise.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: f973819
- Beleg: `inactive_time_does_not_start_the_reminder_rhythm`, `cargo test -p lotse-core` (26/26)
- Offen: –

## E-005 – Ghostty-Tabs, Claude Desktop und Codex erkennen
- Problem: Lotse soll die acht Ghostty-Tabs sowie Claude Desktop und die Codex-Komponenten getrennt zeigen; große Prozesslisten brachen die Erkennung.
- Lösung: Prozessliste wird gestreamt verarbeitet (Test mit 50.000 Zeilen), Codex-Komponenten werden einer Anzeige zugeordnet, Cockpit zeigt die Gruppen getrennt.
- Verworfen: –
- Entscheidung: Agent nach Nutzerwunsch, 2026-10-28
- Live seit: im lokalen Build 0.2.0 unter `/Applications/Lotse.app`
- Beleg: AENDERUNGEN.md A-005, 28 Workspace-Tests
- Offen: Sichtprüfung des laufenden Cockpits mit 8 echten Tabs.

## E-006 – Der normale Bürostuhl ist die Standardhaltung; nach dem …
- Problem: Der normale Bürostuhl ist die Standardhaltung; nach dem Sitzintervall fordert Lotse ausdrücklich zum Haltungswechsel auf.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-006 beschrieben; Abnahmekriterium: Neue und vorhandene Sitz-Zustände erscheinen als „Bürostuhl“. Nach dem Sitzintervall zeigt der Hinweis „Haltung wechseln“ mit Alternativen Stehen und Board.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: d3489ba
- Beleg: Core-Test grün; Workspace-Tests 29/29 und lokales Produktions-Bundle vorhanden. Sichtprüfung des laufenden Cockpits: Bürostuhl als Standard sichtbar.
- Offen: –
## E-007 – Bürostuhl und Swopper sind getrennte Standardhaltungen. Personen …
- Problem: Bürostuhl und Swopper sind getrennte Standardhaltungen. Personen können weitere Haltungen lokal ergänzen oder nicht benötigte entfernen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-007 beschrieben; Abnahmekriterium: Vier Standardhaltungen sind sichtbar. Einstellungen erlauben Namen und Intervall je Haltung, Hinzufügen und Entfernen (mindestens eine Haltung bleibt). Auswahl und Wechselhinweise nutzen die gespeicherte Liste.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: 8d4e37a
- Beleg: 30 Workspace-Tests grün; Tauri-Produktions-Bundle und DMG erstellt. Sichtprüfung: Bürostuhl, Swopper, Stehen und Balance Board sowie Hinzufügen/Entfernen sichtbar.
- Offen: –
## E-008 – Das originale KI-AffAIrs-Logo soll dezent statt des reinen …
- Problem: Das originale KI-AffAIrs-Logo soll dezent statt des reinen Textzugs in der Cockpit-Kopfzeile erscheinen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-008 beschrieben; Abnahmekriterium: Kopfzeile zeigt das vorhandene Original-Logo sowie die Produktkennung „Lotse“; keine neue oder nachgezeichnete Markenvariante.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-28; Umsetzung durch Agent
- Live seit: 91f1ccc
- Beleg: Frontend- und Tauri-Produktions-Build erfolgreich; laufendes Cockpit zeigt die Original-Bildmarke neben „Lotse“.
- Offen: –
## E-009 – Zeitbezogene Übungen per Startknopf mit Countdown und hörbarem …
- Problem: Zeitbezogene Übungen per Startknopf mit Countdown und hörbarem Abschluss durchführen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-009 beschrieben; Abnahmekriterium: Augenhinweis startet 20 s; Pausen nutzen konfigurierte Dauer; laufende Timer bleiben sichtbar, sind abbrechbar und werden nicht durch Hinweise ersetzt. Abschluss einmalig sichtbar und hörbar (abschaltbar).
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: e5a09a5
- Beleg: Core-Dauertest, nativer Countdown-Test und Ego Lite: 20 s bis Fertig, stummer Pausentimer/Abbruch, Karte passt in 380 px; 2026-10-29
- Offen: –
## E-010 – Hochwertige, zusammengehörige eigene Klänge statt Systemton
- Problem: Hochwertige, zusammengehörige eigene Klänge statt Systemton.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-010 beschrieben; Abnahmekriterium: Lokal eingebundene, weich ein-/ausgeblendete Stereo-Klänge für Hinweis, Start und Abschluss; Klangprobe und Stummschaltung; kein Glass mehr.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: e5a09a5
- Beleg: Eigene Stereo-WAVs (-12 dBFS Peak, Ende bei null); Ego-Klangprobe lädt 776204 Byte; genau ein Start-/Abschlussaufruf. Subjektive Klangabnahme durch Nutzer offen
- Offen: Subjektive Klangabnahme durch Claus.
## E-011 – Arbeitslog während der Umsetzung führen
- Problem: Arbeitslog während der Umsetzung führen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-011 beschrieben; Abnahmekriterium: Wünsche, Entscheidungen und Prüfstand sind in ARBEITSLOGBUCH.md nachvollziehbar.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: e5a09a5
- Beleg: Projektlog fortlaufend ergänzt, globaler Eintrag 2026-10-29 geschrieben
- Offen: –
## E-012 – Geöffnetes Dashboard über Dock und ⌘Tab wiederfinden
- Problem: Geöffnetes Dashboard über Dock und ⌘Tab wiederfinden.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-012 beschrieben; Abnahmekriterium: Öffnen aktiviert den regulären macOS-App-Modus; Schließen versteckt das Dashboard und lässt Lotse in der Menüleiste weiterlaufen.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: e5a09a5
- Beleg: Native AX-Abfrage: Cockpit geöffnet background-only=false/visible=true; geschlossen background-only=true/visible=false
- Offen: –
## E-013 – App über Apple Developer freigeben
- Problem: App über Apple Developer freigeben.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-013 beschrieben; Abnahmekriterium: Lokales Paket mit Developer ID signieren, Apple-Notarisierung erfolgreich, Ticket anheften und Gatekeeper prüfen; fehlende Identität/Zugang konkret ausweisen.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: ade59bb
- Beleg: Apple Accepted für App und DMG; beide Logs ohne Issues. Tickets, Signaturen und Gatekeeper am Bundle, entpackten ZIP und App im schreibgeschützt gemounteten DMG erfolgreich, 2026-10-29.
- Offen: –
## E-014 – Fertige signierte App im bestehenden Repository zum Download anbieten
- Problem: Fertige signierte App im bestehenden Repository zum Download anbieten.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-014 beschrieben; Abnahmekriterium: GitHub-Release mit geprüftem, notarisiertem DMG/ZIP; heruntergeladene Datei hat denselben Hash wie die freigegebene Quelle. Repository-Sichtbarkeit unverändert.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: f1244a5
- Beleg: Release lotse-0.1.0 veröffentlicht; DMG/ZIP/SHA256SUMS erneut über GitHub heruntergeladen und SHA-256 identisch. Heruntergeladenes DMG besteht codesign/stapler, 2026-10-29.
- Offen: –
## E-015 – Download direkt im README verlinken
- Problem: Download direkt im README verlinken.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-015 beschrieben; Abnahmekriterium: Prominenter Direktlink öffnet das hochgeladene DMG; Apple-Silicon/macOS-Voraussetzung und Installation erklärt, Aussage zu fehlenden Downloads entfernt.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: f1244a5
- Beleg: Remote-README auf main gelesen: Direktlink stimmt mit veröffentlichtem DMG-Asset überein. Deutsche/englische Installationshinweise aktualisiert.
- Offen: –
## E-016 – README auf neue Funktionen abgleichen
- Problem: README auf neue Funktionen abgleichen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-016 beschrieben; Abnahmekriterium: README auf GitHub enthält Augen-/Pausentimer, drei Klangproben, Haltungsverwaltung, Aktivitätsrhythmus und Dock/⌘Tab sowie bestehenden Download.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: 2cdf97c
- Beleg: Remote-README auf main bytegleich zur lokal geprüften Fassung, inklusive Dock/⌘Tab, 20 s und drei Klängen; 2026-10-29.
- Offen: –
## E-017 – Nutzung und Weitergabe des Originalprodukts lizenzieren; …
- Problem: Nutzung und Weitergabe des Originalprodukts lizenzieren; veränderte Veröffentlichungen unter eigener Identität erlauben.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-017 beschrieben; Abnahmekriterium: LICENSE/NOTICE, klarer Umfang und Marken-/Identitätsabgrenzung, Metadaten und README; Lizenz dem bestehenden Release zugeordnet.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: 1297a97
- Beleg: LICENSE/NOTICE/BRANDING.md auf GitHub bytegleich geprüft; alle drei Release-Anhänge erneut heruntergeladen und verglichen, Metadaten Apache-2.0.
- Offen: –
## E-018 – Selbstbau dokumentieren, Verwendung von Claus' Entwickleridentität …
- Problem: Selbstbau dokumentieren, Verwendung von Claus' Entwickleridentität für Änderungen ausschließen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-018 beschrieben; Abnahmekriterium: Anleitung verlangt eigene App-Kennung, bei veröffentlichten Änderungen eigener Name/Marke und eigene Signierung/Notarisierung; Signierskript nutzt keine feste Claus-Identität als Default.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: 1297a97
- Beleg: Eigene Bundle-ID in DE/EN-Anleitung; kein fester Claus-Identitäts-/Profil-Default im Skript. Drei Tests rot → grün; Remote-Skript geprüft.
- Offen: –
## E-019 – Repository mit Codex Security prüfen; Datenspeicherung, …
- Problem: Repository mit Codex Security prüfen; Datenspeicherung, Zugriffswege, Verschlüsselung, macOS-Rechte, Prozesse, AppleScript, Secrets, Signierung und Abhängigkeiten offenlegen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-019 beschrieben; Abnahmekriterium: Versiegelter, quellbelegter Bericht nennt Findings, Gegenbelege, tatsächlichen Speicherort/-schutz und verbleibende Prüflücken.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: 0dd35bd (Prüfauftrag, Bericht)
- Beleg: Codex-Security-Scan `4eb909c1-564f-4aaa-8d1c-a7a978d4af62`: 1× hoch, 2× niedrig; Bericht erzeugt. RustSec-Abgleich ausdrücklich offen, da Dependabot deaktiviert und keine lokale Advisory-Datenbank vorhanden.
- Offen: Aktueller RustSec-Abgleich.
## E-020 – Beschlossenen Härtungsplan vollständig umsetzen
- Problem: Beschlossenen Härtungsplan vollständig umsetzen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-020 beschrieben; Abnahmekriterium: Alle drei validierten Findings sind mit Regressionstests behoben; lokale Daten werden geordnet und mit `0700`/`0600` gespeichert; Release-Secrets liegen hinter manueller Freigabe; GitHub-Sicherheitskontrollen und PR-Pflicht sind aktiv und nachgewiesen.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: 9f95ac8 (PR #1)
- Beleg: PR #1 gemergt (`9f95ac8`); beide Pflichtchecks grün. Datenrechte, Actions-SHA-Pflicht, Environment-Freigabe, Branch-Schutz, Secret Scanning/Push Protection, Dependabot und private Meldungen remote nachgewiesen. Folge-PR #10 bewahrt den exakten `CFBundleExecutable`-Namen im Universal-Build; echter Developer-ID-Signaturtest grün. Environment ist bewusst noch ohne Apple-Secrets und blockiert Releases bis zur einmaligen sicheren Einrichtung.
- Offen: –
## E-021 – Ausfall untersuchen: Klänge liefen, aber Hinweise fehlten und das …
- Problem: Ausfall untersuchen: Klänge liefen, aber Hinweise fehlten und das Dashboard blieb schwarz; Neustart half. Klären, ob ignorierte/weggeklickte Aufforderungen die Ursache sein können, und belegte Ursache beheben.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-021 beschrieben; Abnahmekriterium: Logs/Crashdaten und Hinweis-/Webview-Zustandsfluss sind ausgewertet; Wegklicken/Ignorieren ist per Regressionstest sicher; bei einem UI-Ausfall bleibt Lotse wiederherstellbar und zeigt eine verwertbare Diagnose statt dauerhaft schwarz zu bleiben.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: c78209e (PR #11)
- Beleg: Nutzer-Reproduktion nach erstem Fix belegte die Ursache: beide WebViews scheiterten mit `-1004`, weil der neue Universal-Build ohne `custom-protocol` weiterhin `devUrl` lud. Wiederholtes Schließen idempotent; Recovery-Härtung bleibt zusätzlich aktiv. Final installierte App lädt Cockpit/Zeitleiste ohne Dev-Server.
- Offen: –
## E-022 – Fokus und Pause sollen in der Anzeige eine konsistente Farblogik haben
- Problem: Fokus und Pause sollen in der Anzeige eine konsistente Farblogik haben.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-022 beschrieben; Abnahmekriterium: Gleiche semantische Zustände verwenden im gesamten Dashboard dieselben Farbrollen; Fokus/Aktivität und Pause sind unterscheidbar, ohne widersprüchliche Grün-/Warnfarben.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: c78209e (PR #11)
- Beleg: Fokus nutzt den konfigurierten Engine-Level; Pause durchgehend Cyan für Ring, Modus, Chip und Aktion. UI-Test sowie Ego-Lite-Fokus-/Pausenansicht ohne Overflow grün.
- Offen: –
## E-023 – Zeiten sehen: was begonnen wurde, wann Pausen stattfanden und wie …
- Problem: Zeiten sehen: was begonnen wurde, wann Pausen stattfanden und wie lange Tasks aktiv laufen.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-023 beschrieben; Abnahmekriterium: Dashboard zeigt für den heutigen Verlauf lokal: erkannte Task-/Projektbezeichnung, Startzeit und aktive Dauer sowie Pausenbeginn/-ende und Dauer; Daten sind begrenzt und enthalten keine Rohpfade oder vollständigen Prozessargumente.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: c78209e (PR #11)
- Beleg: Tageszeitleiste in installierter App; Core-Tests für Start/Dauer/Pausen und pfadfreie Persistenz, Laufzeitdatei `0600`, 3 Tasklabels ohne `/` oder `\\`; max. 50 Tasks/50 Pausen.
- Offen: –
## E-024 – Erneut schwarzes Cockpit und nur hörbare Benachrichtigungen im …
- Problem: Erneut schwarzes Cockpit und nur hörbare Benachrichtigungen im installierten Universal-Build; Fehler muss im Release-Build behoben werden.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-024 beschrieben; Abnahmekriterium: Universal-Build aktiviert Tauri `custom-protocol`; App lädt Cockpit und Toast ohne lokalen Dev-Server; Regressionstest, frischer Build, Apple-Ticket und Zielprüfung unter `/Applications`. Bestehenden veröffentlichten Release separat prüfen.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzerkorrektur, 2026-10-29; Umsetzung durch Agent
- Live seit: c78209e ff. (siehe AENDERUNGEN.md)
- Beleg: Red/Green-Test erzwingt Feature in beiden Architekturen; App-Bundle wuchs durch eingebettete Assets. Finaler Prozess ohne Port 1420: keine `-1004`-Fehler, feste Cockpit-/Zeitleistentexte vorhanden; Apple Accepted und Gatekeeper grün. Veröffentlichter `lotse-0.1.0` separat schreibgeschützt gestartet: nicht betroffen.
- Offen: –
## E-025 – Künftige Lotse-Versionen sollen sichere Updates direkt aus den …
- Problem: Künftige Lotse-Versionen sollen sichere Updates direkt aus den offiziellen GitHub Releases beziehen und in der App anbieten.
- Lösung: Umgesetzt wie in AENDERUNGEN.md A-025 beschrieben; Abnahmekriterium: Updater prüft ausschließlich den offiziellen HTTPS-Release-Feed, verifiziert jedes Update kryptografisch mit eingebettetem öffentlichem Schlüssel, bietet eine bewusste Installation mit Neustart an und erzeugt im geschützten Releasepfad Universal-Updatepaket, Signatur und Metadaten. Private Update- und Apple-Schlüssel gelangen weder in App noch Repository. Negative und positive Tests sowie ein lokaler Paket-/Feed-Test bestehen. `0.1.0` wird als einmalig manuell zu aktualisierende Übergangsversion dokumentiert.
- Verworfen: – (nicht gesondert dokumentiert)
- Entscheidung: Wunsch Nutzer, 2026-10-29; Umsetzung durch Agent
- Live seit: 00d5dd0 (PR #13)
- Beleg: PR #13 gemergt (`00d5dd0`); beide Pflichtchecks grün. 8 UI-, 44 reguläre Rust- und 19 Python-/Securitytests; echter signierter Universal-Archivtest akzeptiert Original und verwirft Bit-Manipulation sowie Versions-Rebinding. Updater-Secrets im geschützten Environment gesetzt, nur Namen geprüft. Apple-Secrets bleiben absent und der Releasepfad dadurch fail-closed. Kein Tag/Release.
- Offen: –

## E-026 – Gestrige Anpassungen nicht sichtbar
- Problem: Der Autostart-LaunchAgent startete einen veralteten Build aus `target/`.
- Lösung: Ursache belegt, alte Instanz beendet, `/Applications/Lotse.app` gestartet. Autostart-Datei nicht vom Agenten umgeschrieben; der Schalter „Beim Anmelden starten“ in Lotse setzt den Pfad neu.
- Verworfen: Datei `~/Library/LaunchAgents/Lotse.plist` direkt umschreiben (von der Berechtigungsprüfung gesperrt, nicht umgangen).
- Entscheidung: Agent, 2026-10-30
- Live seit: 2026-10-30 (laufende App aus `/Applications`)
- Beleg: AENDERUNGEN.md A-026, Handoff 2026-10-30
- Offen: siehe Nachtrag.
- Nachtrag 2026-11-03: Auf Wunsch von Claus die Lotse-Versionen gezählt (eine installierte App `/Applications/Lotse.app` 0.2.0, vier Build-Bundles in `target/`). Die drei Bundles vom 2026-10-29 (`target/universal-apple-darwin/…`, `target/lotse-arm64/…`, `target/lotse-x86_64/…`) gelöscht, `target/release/bundle/macos/Lotse.app` und `/Applications/Lotse.app` behalten, Lotse aus `/Applications` gestartet. Beobachtung: `~/Library/LaunchAgents/Lotse.plist` existiert nicht mehr, Autostart ist derzeit aus. Damit startet kein veralteter Build mehr automatisch; Lotse startet aber auch nicht beim Anmelden, bis der Schalter wieder eingeschaltet wird.

## E-027 – Laufzeit-Anzeigen
- Problem: Anzeigen sollen zeigen, wie lange Fokus, Pause und Haltung schon laufen.
- Lösung: Startzeit und sekündlich laufende Uhr in Fokus-, Pause- und Haltungskarte; laufende Aufgaben und Pausen zählen in der Zeitübersicht live.
- Verworfen: –
- Entscheidung: Agent nach Nutzerwunsch, 2026-10-30
- Live seit: ce807fb, installiert als lokaler Build 0.2.0 (2026-10-30)
- Beleg: Screenshot „Fokus läuft seit 11:29 · 15:41“, UI- und Core-Tests rot→grün
- Offen: –
- Nachtrag 2026-11-03: Die Lauf-Zeile unter der Fokus-Karte entfällt zugunsten der Uhr im Ring und der Startzeit in der Liste (A-031, A-032). Der Stand vom 2026-10-30 oben bleibt erhalten.

## E-028 – Tag beenden
- Problem: Den Arbeitstag beenden, protokollieren und Lotse schließen.
- Lösung: „Tag beenden“ in Kopfzeile (mit Rückfrage), Menüleiste und Hinweis; Tagesprotokoll wird in die Historie geschrieben (Upsert, kein Doppeleintrag), danach beendet sich Lotse; Takt-Sperre `ENDING_DAY` gegen Rennen.
- Verworfen: –
- Entscheidung: Agent nach Nutzerwunsch, 2026-10-30
- Live seit: ce807fb, lokaler Build 0.2.0
- Beleg: 7 Core-Tests rot→grün
- Offen: Echter Klick samt Prozessende noch nicht ausgeführt.
- Nachtrag 2026-11-03: Live ausgelöst um 13:51 (Build fc1db62). Historie geschrieben, Prozess beendet, kein Absturzbericht. Offen: –

## E-029 – Tagesprotokolle sichtbar
- Problem: Sichtbar halten, wie lange Aufgaben und Agenten liefen.
- Lösung: Verlauf je Tag aufklappbar mit Aufgaben/Agenten, Beginn, Dauer und Pausen; Granularität je Projekt/App/Agent.
- Verworfen: je einzelnem Chat (Lotse sieht Chats nicht zuverlässig, nur Prozesse und Fenster).
- Entscheidung: Agent, 2026-10-30; Bestätigung der Granularität durch Claus offen
- Live seit: ce807fb, lokaler Build 0.2.0
- Beleg: `ending_the_day_logs_it_in_the_history_and_stays_resumable`
- Offen: Claus bestätigt, ob Projekt/Agent reicht.

## E-030 – Versionierung
- Problem: Eine vernünftige Versionierung als Voraussetzung für funktionierende Updates.
- Lösung: Eine Quelle (`[workspace.package] version` in `Cargo.toml`), `scripts/versioning.py` und `set-version.sh` für alle fünf Stellen, Changelog je Release, Release nur mit höherer Version als jeder Tag, Build-Marker `LOTSE_BUILD`, Ablauf in `docs/RELEASE.md`. Erster Release wird 0.3.0.
- Verworfen: Release 0.2.0 (würde der gleichnamigen installierten App nie als Update angeboten).
- Entscheidung: Agent nach freigegebenem Plan, 2026-10-30
- Live seit: ed2f72f, noch nicht veröffentlicht
- Beleg: 32 Python-, 13 Rust-, 16 UI-Tests
- Offen: `validate-release.sh` und `build-local.sh` real ausführen; Versionsschema durch Claus bestätigen.

## E-031 – Pausenuhr im Ring
- Problem: Im Pausenmodus zeigte der Ring nur „Pause / läuft“, beim Fokus dagegen die Minuten.
- Lösung: `focusVisual` liefert im Pausenmodus `clockFrom`; der Ring zeigt die sekündlich laufende Pausenuhr mit Untertitel „Pause“.
- Verworfen: zusätzliche Uhr in der Kopfzeilen-Pille (mehr Elemente statt weniger).
- Entscheidung: Agent nach Nutzerwunsch, 2026-11-03
- Live seit: Commit „Cockpit-Übersicht (A-031 bis A-034)“, noch nicht installiert
- Beleg: UI-Test `focusVisual`, Screenshot `?pause=1`
- Offen: Sicht in der installierten App.

## E-032 – „Fokus seit“ statt Lauf-Zeile
- Problem: Die laufende Zeit stand doppelt (Ring und Lauf-Zeile), die Zeile „Pausen heute · davon lang“ war für den Moment wenig hilfreich.
- Lösung: Lauf-Zeile entfernt; die Liste zeigt „Fokus seit HH:MM“ bzw. „Pause seit HH:MM“. Die Zahl der Pausen bleibt in „Heute · Aufgaben & Pausen“ und in „Letzte Tage“ sichtbar.
- Verworfen: nur „davon lang“ streichen (Pausenzahl bliebe doppelt).
- Entscheidung: Agent nach Nutzerwunsch, 2026-11-03
- Live seit: wie E-031
- Beleg: UI-Test `sinceFact`, Screenshots
- Offen: Sicht in der installierten App.

## E-033 – Haltungsampel
- Problem: Die Haltungskarte zeigte die Verteilung nur als graue Leiste und Text, ohne Bewertung.
- Lösung: `postureTraffic` bewertet den Tag. Bürostuhl: Rot, wenn weniger als 10 % Wechsel und das Sitzintervall überschritten ist; Grün ab 33 % Wechsel (Sitz-Steh-Rhythmus 30/15); sonst Orange. Andere Haltungen: Orange, solange ungenutzt oder unter ihrer eingestellten Dauer; Grün, sobald die eingestellte Dauer heute erreicht ist. Pillen ersetzen Split-Bar und Notizzeile; darunter „Wechsel heute x % · Ziel 33 %“.
- Verworfen: Split-Bar zusätzlich behalten (Übersicht); Rot sofort zu Tagesbeginn.
- Entscheidung: Agent nach Nutzerwunsch, 2026-11-03; Schwellen als Annahme, Bestätigung durch Claus offen
- Live seit: wie E-031
- Beleg: 4 UI-Tests, Screenshots
- Offen: Schwellen bestätigen; Sicht in der installierten App.

## E-034 – Entwicklung in „Letzte Tage“
- Problem: Es fehlte, ob Gewohnheiten über die Tage besser werden oder inkonsequent bleiben; die Augenpause (20-20-20) wurde nirgends erfasst.
- Lösung: Der Core zählt Augenpausen je Tag (angeboten, sobald der Hinweis gewählt wird; erledigt, sobald der 20-Sekunden-Timer abläuft, je Hinweis einmal; Wegklicken zählt nicht). Alte Tage haben `None` und gelten nicht als verfehlt. Der Bereich „Entwicklung“ zeigt für die letzten 7 abgeschlossenen Tage (heute ausgenommen, Tage unter 60 min aktiv ausgenommen) drei Gewohnheiten: Augenpausen (mind. die Hälfte gemacht), Lotsenregel (zwischen je 2 h Arbeit eine lange Pause), Feierabend (aktiv nicht länger als der Arbeitstag). Er zeigt jeweils „x von n Tagen“, die Bewertung (konsequent ab 80 %, ausbaufähig ab 50 %, sonst inkonsequent) und den Trend (jüngere gegen ältere Hälfte).
- Verworfen: neue Kennzahlen im oberen Bereich (neuer Umbruch); Trinken und Haltung als Kategorien (stehen schon in der Tabelle).
- Entscheidung: Agent nach Nutzerwunsch, 2026-11-03
- Live seit: wie E-031
- Beleg: 4 Core-Tests, 5 UI-Tests, Screenshot
- Offen: Augenpausen-Daten entstehen erst ab Installation; Tauri-Pfad `timer_tick` → `complete_eyes` ohne eigenen Test; Sicht in der installierten App. Gezählt wird nur, wenn der 20-Sekunden-Timer im Hinweis gestartet wird und abläuft.
