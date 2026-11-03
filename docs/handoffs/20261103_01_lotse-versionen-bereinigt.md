# Handoff — Lotse — 2026-11-03

## Stand
Diese Sitzung bestand bei Lotse nur aus einer Bedienaktion und dem Nachziehen der Projektführung. Am Code hat sich nichts geändert.

- **Versionen bereinigt (Wunsch von Claus):** Installiert ist genau eine App, `/Applications/Lotse.app` mit Version 0.2.0. Dazu lagen vier Build-Bundles in `target/`. Gelöscht habe ich die drei Bundles vom 2026-10-29:
  - `target/universal-apple-darwin/…`
  - `target/lotse-arm64/…`
  - `target/lotse-x86_64/…`

  `target/` ist von Git ausgeschlossen. Behalten: `/Applications/Lotse.app` und `target/release/bundle/macos/Lotse.app` (Stand 2026-10-30). Lotse läuft aus `/Applications`.
- **Beobachtung zu A-026:** `~/Library/LaunchAgents/Lotse.plist` existiert nicht mehr. Autostart ist also aus. Damit startet kein veralteter Build mehr automatisch. Lotse startet aber auch nicht beim Anmelden.
- **Projektführung:**
  - `ENTSCHEIDUNGEN.md` neu angelegt, mit E-001 bis E-030.
  - In der Anforderungstabelle standen Leerzeilen. Dadurch sah die Prüfung nur A-001 bis A-014. Die Leerzeilen sind entfernt.
  - Neue Planschritte 24 bis 30 ordnen A-009 bis A-025 zu.
  - A-010 von „technisch geprüft“ (kein gültiger Status) auf „umgesetzt“ gesetzt, weil die Klangabnahme durch Claus fehlt.
- **IDs:**
  - Geprüft bleiben A-001 bis A-004, A-006 bis A-009, A-011 bis A-025 und A-027.
  - Offen sind A-005, A-010, A-026, A-028, A-029 und A-030.

Entscheidungen in Kurzform:
- A-026 → E-026: Autostart nicht direkt umschreiben, sondern über den Schalter in Lotse neu setzen · Agent, 2026-10-30 · Nachtrag 2026-11-03: Autostart derzeit aus
- A-010 → E-010: eigene Stereo-Klänge statt Systemton · Wunsch 2026-10-29 · live e5a09a5, Klangabnahme offen

## Geänderte Dateien
- `ENTSCHEIDUNGEN.md`: neu. E-005 und E-026 bis E-030 ausführlich. E-001 bis E-025 aus den Belegen und Commits nachgezogen; damals verworfene Alternativen sind nicht dokumentiert.
- `AENDERUNGEN.md`: Leerzeilen in der Tabelle entfernt, A-010 auf „umgesetzt“, Revision A-010, Abschnitt „Abschlussabgleich 2026-11-03“.
- `PLAN.md`: Schritte 24 bis 30 ergänzt, nächster Schritt um Autostart (A-026) und Klangabnahme (A-010) ergänzt.
- `ARBEITSLOGBUCH.md`: Eintrag „Versionen bereinigt, Projektführung nachgezogen“.
- `docs/handoffs/20261103_01_lotse-versionen-bereinigt.md`: dieses Handoff.
- Außerhalb von Git: drei gelöschte Build-Bundles in `target/`.

## Verifikation
- Getestet:
  - `arbeitsstandard.py check .` ohne Befund.
  - `pgrep -fl Lotse` zeigt `/Applications/Lotse.app/Contents/MacOS/lotse`.
  - `Info.plist` der installierten App zeigt Version 0.2.0.
  - Nach dem Löschen findet `find /Applications target -iname Lotse.app` nur noch die zwei behaltenen Bundles.
- Nicht getestet:
  - Keine Rust-, UI- oder Python-Tests in dieser Sitzung (kein Code geändert). Letzter grüner Stand steht im Handoff vom 2026-10-30.
  - `arbeitsstandard.py verify` hat keinen Verify-Befehl definiert.
  - Autostart nach dem Wiedereinschalten.
  - Sichtprüfung A-005, Klick auf „Tag beenden“ (A-028), Release-Skripte live (A-030).
- Secrets-Check: Diff und neue Dateien mit dem Muster aus dem wrapup-Skill durchsucht, kein Fund.
- PersonalOS-Rückschreibung: ja, Aktivitätsnotiz `daily/2026/2026-11-03/activity/01a1a0f3-472d-7fb4-b785-d7f92ceb7921.md` und Tagesrecord, PersonalOS-Commit f707a73 (lokal, kein Push).

## Git-Status
Branch `claude/20261030-laufzeit-tagesende`. Der Commit mit diesem Handoff folgt auf 5a6a96d. Nicht gepusht, kein PR. Die sechs ungetrackten `docs/handoffs/AUTO_20261029_*.md` sind Sicherheitsnetz-Dateien früherer Kompaktierungen. Sie bleiben unverändert und werden nicht committet.

## Offene Folgeschritte
1. **A-026:** Claus schaltet in den Lotse-Einstellungen „Beim Anmelden starten“ wieder ein. Danach prüfen, dass `~/Library/LaunchAgents/Lotse.plist` auf `/Applications/Lotse.app` zeigt.
2. **A-010:** Claus bewertet die eigenen Klänge (subjektive Abnahme).
3. **A-028:** „Tag beenden“ einmal real auslösen, am besten am Tagesende, und Protokoll und Programmende prüfen.
4. **A-029:** Claus klärt die Granularität: reicht Projekt/Agent, oder soll jeder Chat/Tab einzeln erscheinen?
5. **A-030:** `validate-release.sh` und `build-local.sh` real ausführen. Claus bestätigt das Versionsschema mit erstem Release 0.3.0.
6. **A-005:** Sichtprüfung des Cockpits mit 8 echten Ghostty-Tabs sowie Claude Desktop und Codex.
7. **Nur auf ausdrücklichen Auftrag:** Push und PR der Branch, Apple-Secrets, Release 0.3.0 und Live-Update-Test (siehe Handoff 20261030_01).

## Sources
- `ARBEITSLOGBUCH.md`, Eintrag vom 2026-11-03
- `docs/handoffs/20261030_01_tagesabschluss-lotse-laufzeit-versionierung.md`
- Globales Arbeitslogbuch: zwei Einträge vom 2026-11-03 (Versionen geprüft, Bundles bereinigt)
- Commits fbc544a bis 5a6a96d

## Re-entry
Lies dieses Handoff, ZIEL.md, AENDERUNGEN.md, ENTSCHEIDUNGEN.md und PLAN.md, dann weiter mit dem ersten offenen Folgeschritt.
