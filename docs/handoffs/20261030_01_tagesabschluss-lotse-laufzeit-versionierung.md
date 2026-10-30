# Handoff — Lotse: Laufzeit-Anzeigen, Tag beenden und Versionierung — 2026-10-30

## Stand

- **Ursache „gestrige Änderungen nicht sichtbar“ (A-026):** Der Autostart-LaunchAgent `~/Library/LaunchAgents/Lotse.plist` zeigte auf einen veralteten Build in `target/release/bundle/macos/` (Stand 2026-10-29 08:30, ohne `break_timeline`). Der Prozess lief seit 07:56. Ich habe die alte Instanz beendet und `/Applications/Lotse.app` gestartet. Die Autostart-Datei habe ich nicht umgeschrieben: Die Berechtigungsprüfung hat das gesperrt, und das habe ich nicht umgangen. Claus muss in den Lotse-Einstellungen „Beim Anmelden starten“ einmal aus- und wieder einschalten.
- **Laufzeit-Anzeigen (A-027, geprüft):** Fokus-, Pause- und Haltungskarte zeigen Startzeit und eine sekündlich laufende Uhr. Laufende Aufgaben und Pausen zählen in der Zeitübersicht live. Der Screenshot der installierten App zeigt „Fokus läuft seit 11:29“ und „Swopper seit 11:29“.
- **Tag beenden (A-028, umgesetzt):** Kopfzeilen-Button mit Rückfrage, Menüleisten-Eintrag und Hinweis-Aktion „Tag beenden“. Die Engine schreibt das Tagesprotokoll (Start, Ende, Aufgaben, Pausen, Summen) in die lokale Historie, danach beendet sich die App. Aktivität nach dem Beenden öffnet den Tag wieder. Ein Flag verhindert ein Rennen mit dem Takt-Thread.
- **Tagesprotokolle (A-029, umgesetzt):** Der Verlauf zeigt je Tag aufklappbar alle Aufgaben mit Beginn und Dauer sowie die Pausen. Erfasst wird je Projekt/App/Agent, nicht je einzelnem Chat.
- **Versionierung (A-030, umgesetzt):** Quelle ist `[workspace.package] version` in `Cargo.toml`. `scripts/versioning.py` (`check`, `set`, `changelog`, `newer`) mit Wrapper `scripts/set-version.sh` schreibt alle fünf Stellen. `validate-release.sh` verlangt Konsistenz, Changelog-Abschnitt und eine höhere Version als jeder veröffentlichte Tag. Der Release-Text kommt aus `CHANGELOG.md`. Der Build-Marker `LOTSE_BUILD` unterscheidet Release, lokalen Build und CI-Build in Einstellungen und Fußzeile. `scripts/build-local.sh` baut, signiert und installiert lokal und liest den Autostart-Pfad nur.
- **Bewusste Abweichung vom Plan:** Die Version in den Dateien bleibt bis zum Release `0.2.0`, und `build-local.sh` wurde nicht ausgeführt. So bleibt die installierte lokale 0.2.0 für den Live-Update-Test 0.2.0 → 0.3.0 erhalten. Die installierte App hat den neuen Build-Marker noch nicht.
- **Antwort auf „Funktioniert das mit den Updates?“:** Noch nicht. Gründe: 0.1.0 hat keinen Updater, es gibt keinen Release und kein `latest.json`, die Apple-Secrets fehlen bewusst, und der Live-Update-Pfad wurde nie getestet. Ein späterer Release `0.2.0` würde der gleichnamigen lokalen App nie angeboten; deshalb ist der erste Release `0.3.0` geplant.
- **IDs:** A-027 geprüft. A-026, A-028, A-029 und A-030 sind umgesetzt, aber nicht geprüft (Gründe unter „Offene Folgeschritte“). A-005 bleibt unverändert offen.

## Geänderte Dateien

- `core/src/engine.rs`: `DaySummary` mit Startzeit, Ende, Aufgaben und Pausen; `day_closed_at`; `end_day`, `day_summary`, `store_day_summary`; neue Snapshot-Felder für Laufzeiten; 9 neue Tests.
- `src-tauri/src/lib.rs`: `end_day_and_quit`, Flag `ENDING_DAY`, Aktion `day_end`, Menüeintrag „Tag beenden und Lotse schließen“, Befehl `get_build_info`.
- `src-tauri/src/build_info.rs`, `src-tauri/build.rs`: Build-Marker aus `LOTSE_BUILD` mit Zeichenfilter; 3 Tests.
- `ui/src/dashboard-model.ts`, `dashboard.ts`, `dashboard.css`, `api.ts`, `dashboard-model.test.ts`: Laufuhren, Tag-beenden-Dialog, Tagesprotokoll, Versionsanzeige, 8 neue Tests.
- `scripts/versioning.py`, `set-version.sh`, `build-local.sh` (neu); `validate-release.sh`, `ci-release.sh`, `build-universal.sh` und `.github/workflows/release.yml` angepasst.
- `tests/test_version_consistency.py` (neu, 13 Tests), `CHANGELOG.md` (neu).
- `docs/RELEASE.md`, `docs/ENTWICKLUNG.md`, `README.md`: Versionierung und lokaler Build.
- `AENDERUNGEN.md`, `PLAN.md`, `ZIEL.md`, `ARBEITSLOGBUCH.md`, `learning.md`: A-026 bis A-030, Abschlussabgleich, Kandidat zum Autostart-Pfad.

## Verifikation

- Getestet (frisch, vor dem Commit `ed2f72f`): `python3 -m unittest discover -s tests` (32 grün), `cargo test --workspace` (Core 43, Tauri 13 grün), `npm run test:ui` (16 grün), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `npm run build`. Die neuen Tests waren zuvor rot (teils als Kompilierfehler wegen fehlender API, so im Logbuch benannt).
- Getestet: Screenshot der installierten App mit den Laufuhren; `state.json` hat Rechte `0600` und erfasst Aufgaben.
- Nicht getestet: Klick auf „Tag beenden“ (hätte den laufenden Tag beendet); `build-local.sh` und `validate-release.sh` live (nur Syntax und Vertragstests); Autostart nach Neusetzung; Anzeige des Build-Markers in der installierten App; Universal-Build, Notarisierung, Release und der echte Update-Weg.
- Secrets-Check: Diff und neue Dateien durchsucht, keine Zugangsdaten, keine feste Entwickleridentität im Repository.
- PersonalOS-Rückschreibung: ja, Aktivitätsnotiz für 2026-10-30 unter `daily/2026/2026-10-30/activity/` (lokal committet, kein Push).

## Git-Status

- Branch `claude/20261030-laufzeit-tagesende`, Commits `ce807fb` (A-026 bis A-029), `a8150a8` (A-030 erfasst), `ed2f72f` (Versionierung) und der Abschluss-Commit dieses Handoffs. Kein Push, kein PR.
- Sechs ungetrackte `docs/handoffs/AUTO_20261029_*.md` sind Sicherheitsnetz-Dateien früherer Kompaktierungen; sie wurden nicht verändert und nicht committet.
- Installiert ist `/Applications/Lotse.app` als lokaler 0.2.0-Build mit den neuen Features, Developer-ID-signiert, nicht notarisiert. Die notarisierte 0.1.0 liegt gesichert im Scratchpad der Sitzung (`.../scratchpad/backup/Lotse-0.1.0-notarisiert.app`); der Pfad ist nach Sitzungsende unter Umständen nicht mehr vorhanden.

## Offene Folgeschritte

1. **A-026:** Claus schaltet in den Lotse-Einstellungen „Beim Anmelden starten“ aus und wieder ein. Danach prüfen, dass `Lotse.plist` auf `/Applications/Lotse.app` zeigt.
2. **A-028:** „Tag beenden“ einmal real auslösen, am besten am Tagesende, und prüfen, dass das Protokoll im Verlauf steht und die App sich beendet.
3. **A-029:** Rückfrage an Claus: Reicht die Granularität Projekt/Agent, oder soll jeder Chat/Tab einzeln erscheinen? Soll das Tagesprotokoll zusätzlich ins Arbeitslogbuch? Lotse selbst ruft nie `worklog.py` auf.
4. **A-030:** `validate-release.sh` und `build-local.sh` real ausführen (Tag und `origin/main` beziehungsweise bewusste Installation). Claus bestätigt das Versionsschema: Dev-Kennzeichnung nur in der Anzeige, erster Release 0.3.0.
5. **A-005:** Sichtprüfung des 8-Tab-Cockpits, unverändert offen.
6. **Nur auf ausdrücklichen Auftrag, Reihenfolge:** (a) Push und PR der Branch, geschützter Merge nach `main`; (b) Apple-Secrets im Environment `production-release` einrichten (`docs/RELEASE.md`); (c) `scripts/set-version.sh 0.3.0`, Changelog-Datum, Tag `v0.3.0`, Release-Workflow starten, Draft prüfen, veröffentlichen; (d) Live-Update-Test 0.2.0 → 0.3.0 mit Download, Signaturprüfung, Neustart und erhaltener `state.json`.
7. Den privaten Updater-Schlüssel zusätzlich in eine geschützte, vom Owner kontrollierte Sicherung aufnehmen (offen aus dem Handoff vom 2026-10-29).

## Sources

- Commits `ce807fb`, `a8150a8`, `ed2f72f` auf `claude/20261030-laufzeit-tagesende`.
- `ARBEITSLOGBUCH.md`: Einträge „Laufzeit-Anzeigen und Tag beenden (A-026 bis A-029)“ und „Versionierung umgesetzt (A-030)“.
- Freigegebener Plan: `/Users/kiaffairs/.claude/plans/plan-schreiben-bitte-tender-wombat.md`.
- Vorheriges Handoff: `docs/handoffs/20261029_01_tagesabschluss-lotse-sichere-updates.md`.
- Globales Arbeitslogbuch: zwei Einträge vom 2026-10-30 (Features, Versionierung) plus Tagesabschluss.

## Re-entry

Lies dieses Handoff, ZIEL.md, AENDERUNGEN.md und PLAN.md, dann weiter mit dem ersten offenen Folgeschritt.
