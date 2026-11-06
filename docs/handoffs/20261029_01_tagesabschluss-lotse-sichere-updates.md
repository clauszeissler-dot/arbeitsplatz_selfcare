# Handoff — Lotse: Freigabe, Stabilität und sichere Updates — 2026-10-29

## Was ist passiert

- Lotse wurde als teilbares macOS-Produkt fertiggestellt: Menüleistenbetrieb, auffindbares Cockpit in Dock und `⌘Tab`, eigene Klänge, startbare Kurz-Timer, lokale Zeitübersicht für Aufgaben und Pausen sowie konsistente Fokus-/Pausenfarben.
- Version `0.1.0` wurde Developer-ID-signiert, von Apple notarisiert und als Apple-Silicon-DMG/ZIP auf GitHub veröffentlicht. README, deutsche/englische Anleitung, Apache-2.0-Lizenz, Hinweise zu Marke und Entwickleridentität sowie Selbstbauanleitung sind auf `main`.
- Das Sicherheitsaudit führte zur Härtung von Persistenz, Release-Secrets, Signer-/Team-/Bundle-Prüfung, GitHub-Actions, Branchschutz und Repository-Sicherheitsfunktionen. Lokale Arbeitsdaten bleiben unverschlüsselt, aber benutzerexklusiv mit `0700`/`0600`; Rohpfade und vollständige Prozessargumente werden nicht dauerhaft gespeichert.
- Der wiederkehrende schwarze Cockpit-/Hinweiszustand wurde letztlich auf den Universal-Build ohne Tauri-Feature `custom-protocol` zurückgeführt. Der korrigierte Build bindet die Oberfläche ein; der installierte notarisierten Stand zeigte danach Cockpit und Hinweise ohne lokalen Dev-Server. Das Wegklicken oder Ignorieren der Aufforderungen war nicht die Ursache.
- A-025 ergänzt ab Quellversion `0.2.0` sichere GitHub-Updates: fester offizieller HTTPS-Feed, eingebetteter öffentlicher Prüfschlüssel, versionsgebundene Signatur, bewusste Installation und Neustart sowie getrennte Rechte für Cockpit und Hinweisfenster. Der geschützte Releasepfad erzeugt Universal-Archiv, Signatur und `latest.json`.
- Die beiden privaten Updater-Werte liegen außerhalb des Repositories im macOS-Schlüsselbund und als verdeckte Secrets im geschützten GitHub-Environment. Apple-Secrets fehlen dort weiterhin bewusst; der Releasejob bleibt bis zu einem gesondert beauftragten offiziellen Release fail-closed.

## Wo liegen die Dinge

- Projektsteuerung: [ZIEL.md](../../ZIEL.md), [AENDERUNGEN.md](../../AENDERUNGEN.md), [PLAN.md](../../PLAN.md) und [ARBEITSLOGBUCH.md](../../ARBEITSLOGBUCH.md).
- Produkt-/Sicherheitsdokumentation: [README.md](../../README.md), [SECURITY.md](../../SECURITY.md), [Release-Anleitung](../RELEASE.md) und [Selbstbauanleitung](../ANLEITUNG.md).
- Updater: `ui/src/updater.ts`, `ui/src/updater-model.ts`, `src-tauri/tauri.conf.json`, `src-tauri/capabilities/`, `src-tauri/src/updater_security.rs` und `scripts/create-update-artifacts.sh`.
- Releaseautomation: `.github/workflows/release.yml`, `scripts/ci-release.sh` und `scripts/build-universal.sh`.
- GitHub: Produkt-PR [#13](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/pull/13), Abschluss-PR [#14](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/pull/14), veröffentlichter Stand [lotse-0.1.0](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/tag/lotse-0.1.0).
- PersonalOS-Aktivität: [Lotse abgesichert und sichere GitHub-Updates integriert](</Users/kiaffairs/Desktop/personalOS/daily/2026/2026-10-29/activity/01a18923-3b70-7e19-a3be-c0bfa7771201.md>).

## Verifikation

- A-025 lokal: 8 UI-Tests, 44 reguläre Rust-Tests, 19 Python-/Release-/Securitytests, Frontend-Produktionsbuild, Formatierung, Clippy und npm-Audit mit 0 Schwachstellen grün.
- Echter signierter Universal-Archivtest: Original akzeptiert; Bit-Manipulation und Wiederverwendung derselben Signatur für eine andere Version abgelehnt. Lokales Prüf-Bundle `0.2.0` ist `x86_64 arm64` und Developer-ID-signiert, aber nicht als `0.2.0` notarisiert, installiert oder veröffentlicht.
- PR #13: `Kernlogik und Security` grün in 3:44 Minuten; vollständiger unsignierter Universal-Mac-Build grün in 12:36 Minuten. PR #14 wiederholte dieselben Schutzgates für den Dokumentationsabschluss: 3:29 und 13:36 Minuten.
- Remote geprüft: `main` steht auf `a3524133934e20b136c5a3986a71dc841d5f52cd`; beide Feature-Branches sind gelöscht; nur `lotse-0.1.0` ist veröffentlicht. Kein `v0.2.0`-Tag, Draft oder Release.
- Nicht geprüft, weil noch nicht veröffentlicht: echter In-App-Wechsel von einer installierten `0.2.x` auf eine neuere GitHub-Version. Version `0.1.0` besitzt keinen Updater und benötigt einmalig eine manuelle DMG-Aktualisierung.

## Git-Status

- Abschlussbranch: `codex/20261029-lotse-handoff`, lokal von synchronem `main` angelegt; kein Push zum Tagesabschluss.
- Letzter geteilter Commit: `a352413` (`A-025`-Abschlussbelege nach geschütztem Produktmerge). Der Abschlussbranch enthält zusätzlich nur dieses Handoff.
- Keine offenen eigenen Code- oder Produktänderungen außerhalb dieses Handoffs.
- Sechs vorhandene ungetrackte `docs/handoffs/AUTO_20261029_*.md` sind Sicherheitsnetz-Dateien aus früheren Kompaktierungen. Sie wurden weder verändert noch in die Produkt-PRs aufgenommen.

## Offene Folgeschritte

1. Nur mit neuem Veröffentlichungsauftrag: Apple-Environment-Secrets gemäß `docs/RELEASE.md` einrichten, `v0.2.0` vom geschützten `main` erzeugen, den manuell freizugebenden Releaseworkflow starten und den Draft vor Veröffentlichung prüfen.
2. Nach Veröffentlichung den GitHub-Download, Apple-Signatur/Notarisierung, `latest.json`, Tauri-Signatur und den echten In-App-Updatepfad mit einer nachfolgenden Testversion nachweisen. Keine Produktionsversion nur zum Test veröffentlichen.
3. Bestehenden Nutzern klar mitteilen, dass der Wechsel von `0.1.0` auf `0.2.0` einmalig manuell per DMG erfolgt; erst danach funktionieren direkte Updates.
4. Den privaten Updater-Schlüssel zusätzlich in eine geschützte, vom Owner kontrollierte Sicherung aufnehmen. Ohne diesen Schlüssel können bereits installierte Versionen keine künftigen Originalupdates mehr verifizieren.
5. Falls Cockpit oder Hinweise erneut schwarz bleiben: vor Neustart den Zeitpunkt und `ui-diagnostics.json` sichern und gegen WebKit-Logs prüfen; Wegklicken allein nicht als Ursache annehmen.

## Übergabe-Prompt für die nächste Session

> Lies `docs/handoffs/20261029_01_tagesabschluss-lotse-sichere-updates.md`, `ZIEL.md`, `AENDERUNGEN.md`, `PLAN.md` und den jüngsten Abschnitt in `ARBEITSLOGBUCH.md`. Setze nur bei einem neuen Auftrag mit den „Offenen Folgeschritten“ fort. Veröffentliche keinen `0.2.0`-Release, bevor Apple-Secrets, Draft-Prüfung und bewusste Freigabe geklärt sind.
