# Entwicklung

[Zurück zur Übersicht](../README.md)

Voraussetzungen: Xcode Command Line Tools, Rust (stable), Node 22.

```sh
npm install
npm run tauri dev          # App mit Hot Reload
cargo test -p lotse-core   # Logik-Tests (laufen auf jedem Betriebssystem)
npm run dev                # nur Oberfläche im Browser, mit Beispieldaten
```

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
