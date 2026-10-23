# Lotse – by KI AffAIrs

Belastungs-Ampel und Pausen-Lotse für die Arbeit mit KI-Agenten.
Lotse sitzt in der Mac-Menüleiste, liest deine **Ghostty-Tabs** und erkennt, wo
**Claude Code, Codex & Co.** gerade arbeiten. Dazu zählen **ChatGPT** und **Claude Desktop**.
Er gibt dir dezente Hinweise: trinken, Position wechseln (Swopper, Stehen, Balance Board),
Pause nach dem Fluglotsen-Prinzip.

- **Nur Hinweise, kein Overlay.** Eine kleine Karte oben rechts, die nach 40 s von selbst verschwindet.
- **Meeting-Ruhe:** Kamera an oder Mikrofon länger als 2 min ⇒ keine Hinweise.
- **Alles lokal.** Keine Cloud, kein Tracking. Daten liegen in
  `~/Library/Application Support/de.kiaffairs.lotse/`.

## Was Lotse misst

| Quelle | Wie |
|---|---|
| Ghostty-Tabs, Ordner, Fokus | Offizielle AppleScript-API (ab Ghostty 1.3): Fenster → Tabs → Terminals mit `working directory` und `tty` |
| Laufende Agenten pro Tab | Prozesse auf dem `tty` des Tabs (`claude`, `codex`, `gemini`, `aider`, `opencode` …). Er „arbeitet“, wenn CPU ≥ 3 % oder ein Spinner im Titel steht |
| ChatGPT / Claude Desktop | CPU der App-Prozesse; Code-Sessions aus Claude Desktop werden mitgezählt |
| Pausen | Leerlaufzeit von Maus und Tastatur (`ioreg`), Ruhezustand des Macs |
| Meetings | Läuft gerade irgendwo ein Mikrofon oder eine Kamera (CoreAudio/CoreMediaIO, ohne Aufnahme) |

Warum welche Grenze gilt, steht in [docs/RECHERCHE.md](docs/RECHERCHE.md).

## Installation (für dich)

1. DMG aus dem GitHub-Release laden, Lotse nach *Programme* ziehen, starten.
2. Beim ersten Start fragt macOS: **„Lotse möchte Ghostty steuern“** → *OK*.
   (Später unter *Systemeinstellungen → Datenschutz & Sicherheit → Automation*.)
3. Im Cockpit → Einstellungen → **„Beim Anmelden starten“** aktivieren.

Ghostty muss in der **neuesten Version** laufen (mindestens 1.3, wegen der AppleScript-API).

## Entwickeln

Voraussetzungen: Xcode Command Line Tools, Rust (stable), Node 22.

```sh
npm install
npm run tauri dev          # App mit Hot Reload
cargo test -p lotse-core   # Logik-Tests (laufen auf jedem OS)
npm run dev                # nur Oberfläche im Browser, mit Beispieldaten
```

Aufbau:

```
core/        Plattformunabhängig: Ghostty-/ps-/ioreg-Parser, Ampel, Erinnerungs-Engine (+ Tests)
src-tauri/   Mac-App: Menüleiste, Hinweisfenster, Sammeln der Daten, Mikro/Kamera-Erkennung
ui/          Cockpit und Hinweis-Karte (Vite + TypeScript, KI-AffAIrs-Design)
docs/        Recherche, Release-Anleitung
```

## Release

Signieren mit Apple Developer ID und Notarisierung: siehe [docs/RELEASE.md](docs/RELEASE.md).
Kurz: GitHub-Secrets anlegen, dann `git tag v0.1.0 && git push origin v0.1.0`.

## Fahrplan

- [x] Phase 1 – Mac-MVP: Ampel, Hinweise, Ghostty, Agenten, Meetings
- [ ] Phase 2 – Kalibrieren mit echten Daten (1–2 Wochen), Schwellen anpassen
- [ ] Phase 3 – Signierte & notarisierte Releases, Auto-Update
- [ ] Phase 4 – Windows: Windows Terminal per UI Automation, Leerlauf per `GetLastInputInfo`,
      Signierung per Azure Trusted Signing
