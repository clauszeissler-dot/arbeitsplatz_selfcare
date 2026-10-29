![KI AffAIrs – Sicherheit. Vertrauen. Nutzen.](docs/assets/ki-affairs-github-readme-banner.png)

# Lotse – dein Pausen-Lotse für die Arbeit mit KI

**Lotse sagt dir, wann dein Kopf eine Pause braucht.** Die kleine Mac-App sitzt in der Menüleiste.
Sie sieht, wie viele Projekte und KI-Agenten gerade parallel laufen, und zeigt dir das als Ampel.
Sie erinnert dich ans Trinken, an den Wechsel zwischen Sitzen und Stehen und an echte Pausen.

Lotse ist für alle, die viele Stunden am Rechner mit Claude Code, Codex, ChatGPT oder Claude Desktop arbeiten.

## Download für den Mac

**[⬇ Lotse 0.1.0 herunterladen (.dmg)](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/download/lotse-0.1.0/Lotse.dmg)** · [Alle Release-Dateien](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/tag/lotse-0.1.0)

Für **Apple Silicon (M1 oder neuer), ab macOS 13**. Mit Developer ID signiert und von Apple notarisiert.
DMG öffnen und **Lotse** in den Ordner **Programme** ziehen.

📖 **[Anleitung auf Deutsch](docs/ANLEITUNG.md)** · 🇬🇧 **[User guide in English](docs/GUIDE.en.md)**

![Cockpit von Lotse mit Ampel, Trinken, Haltung und laufenden Projekten](docs/assets/cockpit.png)

## Was Lotse für dich tut

- **Ampel in der Menüleiste:** Grün, Gelb oder Rot, je nachdem, wie viel du gerade gleichzeitig überwachst.
- **Erkennt deine Arbeit:** Lotse liest deine Tabs im Terminal Ghostty und sieht, in welchem Tab gerade ein KI-Agent arbeitet. ChatGPT, Claude Desktop und die Codex App zählen mit.
- **Dein Arbeitsrhythmus:** Die Erinnerungen richten sich nach erkannter Aktivität. Der Rhythmus beginnt beim Arbeiten, nicht allein beim App-Start.
- **Trinken:** Lotse erinnert dich alle 45 Minuten an ein Glas, damit du über 8 Stunden auf 2 Liter kommst.
- **Haltung:** Lotse startet mit Bürostuhl, Swopper, Stehen und Balance Board. Jede Person kann ihre Haltungen samt Wechselintervall lokal ergänzen oder entfernen.
- **Pausen wie bei Fluglotsen:** Spätestens nach 75 Minuten ist eine kurze Pause fällig, nach 2 Stunden eine lange Pause. Die Dauern sind einstellbar.
- **Startbare Übungstimer:** Die optionale Augenpause bietet **20 Sekunden in die Ferne schauen**; Pausen nutzen deine eingestellte Dauer. Mit **Starten** beginnt ein sichtbarer Countdown. Er bleibt bis **Fertig** oder **Abbrechen** stehen und meldet den Abschluss auf Wunsch mit einem Klang.
- **Eigene Klangwelt:** Sanfte, lokale Klänge für **Hinweis, Start und Abschluss**. Alle drei kannst du in den Einstellungen probehören. Der Hinweisklang ist abschaltbar; Timer haben einen eigenen Klangschalter.
- **Nur Hinweise, kein Zwang:** Eine kleine Karte oben rechts. Ohne laufenden Timer verschwindet sie nach der eingestellten Sichtdauer (standardmäßig 40 Sekunden).
- **Fenster wiederfinden:** Bei geöffnetem Cockpit erscheint Lotse im **Dock und in ⌘Tab**. Schließt du das Fenster, läuft die App in der Menüleiste weiter.
- **Ruhe im Meeting:** Ist deine Kamera oder dein Mikrofon an, kommen keine Hinweise.

![Hinweis-Karte von Lotse](docs/assets/hinweis.png)

![Countdown der 20-Sekunden-Augenpause](docs/assets/timer.png)

## Gut zu wissen

- **Nur für den Mac:** Lotse läuft ab macOS 13. Eine Windows-Version ist geplant.
- **Alles bleibt auf deinem Rechner:** Lotse schickt keine Daten ins Internet.
- **Keine Garantie für deine Gesundheit:** Lotse ist eine Erinnerungshilfe, kein Medizinprodukt. Die Startwerte beruhen auf Studien und Empfehlungen. Hör trotzdem auf deinen Körper.
- **Woher die Werte kommen:** [Recherche und Quellen](docs/RECHERCHE.md)

## Stand

Lotse ist eine erste Version (0.1.0). Der [fertige Download für Apple Silicon](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/download/lotse-0.1.0/Lotse.dmg) ist signiert und von Apple notarisiert.
Alternativ kannst du die App selbst bauen: [Anleitung](docs/ANLEITUNG.md#installieren).

## Für Entwickler

Aufbau, Tests und Bauen: [docs/ENTWICKLUNG.md](docs/ENTWICKLUNG.md) ·
Signieren und Notarisieren mit Apple Developer ID: [docs/RELEASE.md](docs/RELEASE.md)

---

Ein Projekt von **KI AffAIrs** – Claus Zeißler, Dortmund.
