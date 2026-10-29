# Entwicklung

[Zurück zur Übersicht](../README.md)

Voraussetzungen: Xcode Command Line Tools, die in `rust-toolchain.toml`
festgelegte Rust-Version und Node 22.

```sh
npm install
npm run tauri dev          # App mit Hot Reload
cargo test -p lotse-core   # Logik-Tests (laufen auf jedem Betriebssystem)
npm run dev                # nur Oberfläche im Browser, mit Beispieldaten
```

Auf diesem Mac ist Xcode 27 installiert. Die Projektdatei `.cargo/config.toml`
setzt für macOS automatisch die erforderliche Linker-Option. Für einen lokalen
Build der Host-Architektur bleibt `npm run tauri build` verfügbar. Der
reproduzierbare Universal-Build läuft über `scripts/build-universal.sh`: Das
Skript baut ARM64 und x86_64 per Cargo in getrennten Zielverzeichnissen, lässt
Tauri anschließend nur noch die jeweiligen App-Bundles erzeugen und verbindet
die beiden geprüften Binärdateien mit `lipo`. Dadurch werden Host-Prozedural-
Makros nicht aus einem Cross-Target-Verzeichnis geladen oder zwischen den
Architekturen überschrieben.

## Eigene Builds und Veröffentlichungen

Lizenz: [Apache-2.0](../LICENSE); zusätzliche Marken-/Identitätsabgrenzung:
[BRANDING.md](../BRANDING.md). Für eigene Builds `identifier` und `productName`
wie in der [Selbstbau-Anleitung](ANLEITUNG.md#alternativ-selbst-bauen) überschreiben.
Für dauerhafte Forks diese Werte in `src-tauri/tauri.conf.json` ändern. Für eine
veröffentlichte Änderung auch die Texte/Markengrafik in `ui/src/dashboard.ts`,
`ui/src/toast.ts` und den HTML-Titeln sowie eigene Icons unter `assets/` und
`src-tauri/icons/` anpassen. Ein zusätzlicher Copyright-Hinweis für eigene
Änderungen ist möglich; ursprüngliche erforderliche Urheberhinweise bleiben.

Für eigene Veröffentlichungen sind die Apple-Zugänge, Zertifikate und Secrets des
eigenen Accounts zu verwenden. Die Release-Workflow-Datei enthält nur Secret-Namen;
sie erteilt weder Zugriff auf Claus' Account noch die Berechtigung, seine
Entwickleridentität für geänderte Produkte zu verwenden. Der offizielle
Signierjob ist zusätzlich durch das manuell freizugebende GitHub-Environment
`production-release` geschützt.

## Aufbau

```
core/        Plattformunabhängig: Ghostty-/ps-/ioreg-Parser, Ampel, Erinnerungs-Engine (+ Tests)
src-tauri/   Mac-App (Tauri 2): Menüleiste, Hinweisfenster, Datensammlung, Mikro-/Kamera-Status
ui/          Cockpit und Hinweis-Karte (Vite + TypeScript, KI-AffAIrs-Design)
docs/        Anleitungen, Recherche, Release
```

## Datenquellen

| Quelle | Wie |
|---|---|
| Ghostty-Tabs, Ordner, Fokus | Offizielle AppleScript-API (ab Ghostty 1.3): Fenster → Tabs → Terminals mit `working directory` und `tty` |
| Agenten pro Tab | Prozesse auf dem `tty` des Tabs (`claude`, `codex`, `gemini`, `aider`, `opencode` …). „Arbeitet“ bei CPU ≥ 3 % oder Spinner im Titel |
| ChatGPT / Claude Desktop | CPU der App-Prozesse; Code-Sessions aus Claude Desktop zählen mit |
| Pausen | Leerlaufzeit aus `ioreg` (`HIDIdleTime`), Ruhezustand des Macs |
| Meetings | CoreAudio/CoreMediaIO `DeviceIsRunningSomewhere`, ohne Aufnahme und ohne Freigabe-Dialog |

## Fahrplan

- [x] Phase 1 – Mac-MVP: Ampel, Hinweise, Ghostty, Agenten, Meetings
- [ ] Phase 2 – Kalibrieren mit echten Daten (1–2 Wochen), Schwellen anpassen
- [ ] Phase 3 – Signierte und notarisierte Releases ([RELEASE.md](RELEASE.md)), Auto-Update
- [ ] Phase 4 – Windows: Windows Terminal per UI Automation, Leerlauf per `GetLastInputInfo`, Signierung per Azure Trusted Signing
