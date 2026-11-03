# Handoff — Lotse — 2026-11-03

## Stand
Die Sitzung hatte drei Teile: die laufende App im Hintergrund überwachen, das Cockpit übersichtlicher machen, und zum Tagesende „Tag beenden“ live bestätigen.

- **Wächter:** Von 11:45 bis 13:51 lief alle 30 Sekunden eine Prüfung auf Prozess, Neustart, CPU, Speicher, Takt in `state.json`, Absturzberichte und Fehlerlog. Er hat nur zwei Ereignisse gemeldet, beide erwartet: den Neustart beim Austausch um 13:40 und das Programmende durch „Tag beenden“ um 13:51. Das Skript liegt im Scratchpad der Sitzung und gehört nicht zum Repository. Zum Sitzungsende ist der Wächter beendet.
- **Cockpit-Übersicht (Wunsch von Claus):**
  - A-031: Der Ring zeigt in der Pause die laufende Pausenuhr.
  - A-032: Die Lauf-Zeile unter der Fokus-Karte ist weg. In der Liste steht jetzt „Fokus seit HH:MM“ bzw. „Pause seit HH:MM“ statt „Pausen heute“.
  - A-033: Die Haltungsampel zeigt Pillen je Haltung und ersetzt Split-Bar und Notizzeile.
  - A-034: Der Core zählt Augenpausen (angeboten, und erledigt bei abgelaufenem 20-Sekunden-Timer). Der neue Bereich „Entwicklung“ in „Letzte Tage“ zeigt Augenpausen, Lotsenregel und Feierabend.
  - Unverändert, weil Claus es gelobt hat: „Tag beenden“, Arbeitstag, Pillen an der Ampel, Tagesprotokoll, „Heute“.
- **Installation:** Nachdem Claus gefragt hatte, warum er die Änderungen nicht sieht: Build fc1db62 mit Developer ID signiert (nicht notarisiert) und in `/Applications/Lotse.app` installiert. Die alte App liegt in `target/local-backup/Lotse.app`. Der Tagesstand blieb erhalten.
- **A-028:** „Tag beenden“ lief um 13:51:25 real. Die Historie für 2026-11-03 ist geschrieben (239 min aktiv, 4 Pausen, 13 Aufgaben) und Lotse hat sich beendet. Status jetzt geprüft.
- **Geprüft:** A-001 bis A-004, A-006 bis A-009, A-011 bis A-025, A-027 und A-028.
- **Offen:**
  - A-031 bis A-034: umgesetzt und installiert, die Sichtbestätigung durch Claus fehlt.
  - A-005, A-010, A-026, A-029, A-030.

Entscheidungen in Kurzform:
- A-031 → E-031: Pausenuhr im Ring über `clockFrom` · Agent nach Wunsch, 2026-11-03 · installiert fc1db62
- A-032 → E-032: „Fokus/Pause seit“ statt Lauf-Zeile · Agent nach Wunsch, 2026-11-03 · installiert fc1db62
- A-033 → E-033: Haltungsampel, Ziel 33 % Wechsel, Rot unter 10 % nach überschrittenem Sitzlimit, Grün je Haltung ab eingestellter Dauer · Agent nach Wunsch, 2026-11-03 · Schwellen sind Annahmen, die Claus noch bestätigen muss
- A-034 → E-034: Augenpausen-Zählung plus Entwicklung über 7 abgeschlossene Tage (konsequent ab 80 %, ausbaufähig ab 50 %) · Agent nach Wunsch, 2026-11-03 · installiert fc1db62
- A-027 → E-027: Nachtrag, die Lauf-Zeile entfällt · 2026-11-03
- A-028 → E-028: Nachtrag, live bestätigt · 2026-11-03

## Geänderte Dateien
- `core/src/engine.rs`: Augenpausen-Zähler (`eyes_offered`, `eyes_done`, `eyes_counted_at`), `complete_eyes`, `DaySummary` mit `Option`-Feldern, Snapshot-Felder, 4 neue Tests.
- `src-tauri/src/lib.rs`: `timer_tick` zählt abgelaufene Augen-Timer, und zwar erst nach Freigabe des Übungs-Locks.
- `ui/src/dashboard-model.ts`: `focusVisual` mit `clockFrom`, `sinceFact`, `postureTraffic`, `developmentRows`.
- `ui/src/dashboard-model.test.ts`: 11 neue Tests.
- `ui/src/dashboard.ts`, `ui/src/dashboard.css`: Fokus-Karte, Haltungspillen, Bereich „Entwicklung“.
- `ui/src/api.ts`: Typen und Beispieldaten (Vorschau mit `?pause=1` und `?chair=1`).
- `AENDERUNGEN.md`, `ENTSCHEIDUNGEN.md`, `PLAN.md`, `ARBEITSLOGBUCH.md`: A-031 bis A-034, Nachträge zu A-027 und A-028, Planschritte 31 bis 33, Abschlussabgleich.
- `docs/handoffs/20261103_02_lotse-cockpit-uebersicht.md`: dieses Handoff.
- Außerhalb von Git: `/Applications/Lotse.app` ersetzt, Sicherung unter `target/local-backup/Lotse.app`.

## Verifikation
- Getestet:
  - `cargo test --workspace`: 60 Tests grün. Die 4 neuen Core-Tests waren vorher rot (Kompilierfehler).
  - `cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --all -- --check`: grün.
  - `npm run test:ui`: 27 grün. Die neuen Tests waren vorher rot: fehlender Export, plus ein Fehlschlag bei der Lotsenregel.
  - `npm run build`: grün. `python3 -m unittest discover -s tests`: OK.
  - `npm run tauri -- build --no-sign --bundles app`: Exit 0. `codesign --verify --deep --strict`: grün.
  - Ego-Lite-Screenshots der Designvorschau liegen im Scratchpad der Sitzung (`cockpit-fokus.png`, `cockpit-pause2.png`, `cockpit-entwicklung.png`).
  - Nach der Installation: Prozess lief aus `/Applications`, `state.json` enthält die neuen Felder.
  - `arbeitsstandard.py check .`: ohne Fehler.
- Nicht getestet:
  - Sicht des neuen Cockpits in der installierten App durch Claus.
  - Der Tauri-Pfad `timer_tick` → `complete_eyes` hat keinen eigenen Test.
  - Echte Augenpausen-Zählung im Betrieb: Es gibt noch keine Daten, die Zählung läuft erst seit 13:40.
  - `arbeitsstandard.py verify` hat keinen Verify-Befehl definiert.
  - Notarisierung.
  - Autostart.
- Secrets-Check: Branch-Diff und Arbeitsbaum mit dem Muster aus dem wrapup-Skill durchsucht, kein Fund.
- PersonalOS-Rückschreibung: ja. Aktivitätsnotiz `daily/2026/2026-11-03/activity/01a1a178-817c-7590-813c-816de694eac0.md` und Tagesrecord, `pos-verify` pass, Dashboard aktuell, PersonalOS-Commit f9b9fdd (lokal, kein Push).

## Git-Status
Branch `claude/20261030-laufzeit-tagesende`. Commits dieser Sitzung: fc1db62, 1bcbfe9, afdf96e, dazu der Abschluss-Commit mit diesem Handoff. Nicht gepusht, kein PR. Die sechs ungetrackten `docs/handoffs/AUTO_20261029_*.md` bleiben unverändert und werden nicht committet.

## Offene Folgeschritte
1. **A-031, A-032, A-033, A-034:** Claus sieht sich das neue Cockpit an und bestätigt die Schwellen. Danach den Status auf „geprüft“ setzen. Option, falls gewünscht: „0 %“ statt „ungenutzt“, damit die Haltungspillen kompakter bleiben.
2. **A-026:** Claus schaltet in den Lotse-Einstellungen „Beim Anmelden starten“ wieder ein. Danach prüfen, ob `~/Library/LaunchAgents/Lotse.plist` auf `/Applications/Lotse.app` zeigt.
3. **A-010:** Claus nimmt die eigenen Klänge ab (subjektive Abnahme).
4. **A-029:** Claus klärt die Granularität der Tagesprotokolle: Reicht Projekt/Agent, oder soll jeder Chat/Tab einzeln erscheinen?
5. **A-030:** `validate-release.sh` und `build-local.sh` real ausführen und das Versionsschema bestätigen. Release 0.3.0 nur auf Auftrag.
6. **A-005:** Sichtprüfung des Cockpits mit 8 echten Ghostty-Tabs sowie Claude Desktop und Codex.
7. **Nur auf ausdrücklichen Auftrag:** Push und PR der Branch, Apple-Secrets, Release 0.3.0 und Live-Update-Test. Außerdem das Ego-Lite-Update (`ego-browser upgrade`), das noch auf die Zustimmung von Claus wartet.

## Sources
- ARBEITSLOGBUCH.md: Einträge „Hintergrund-Wächter und Cockpit-Übersicht (A-031 bis A-034)“ und „Tagesende, A-028 live bestätigt“, beide 2026-11-03
- Commits fc1db62, 1bcbfe9, afdf96e
- ENTSCHEIDUNGEN.md E-027, E-028, E-031 bis E-034

## Re-entry
Lies dieses Handoff, ZIEL.md, AENDERUNGEN.md, ENTSCHEIDUNGEN.md und PLAN.md, dann weiter mit dem ersten offenen Folgeschritt.
