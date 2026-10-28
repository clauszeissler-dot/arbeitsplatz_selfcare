# Handoff — Lotse — 2026-10-28

## Was ist passiert

- Lotse ist als lokale macOS-App gebaut und läuft aus `target/release/bundle/macos/Lotse.app`.
- Die App arbeitet aktivitätsbasiert, zeigt visuelle Erinnerungen und besitzt einen bewusst abschaltbaren Systemton.
- Bürostuhl ist die Standardhaltung. Swopper bleibt erhalten; zusätzlich sind Stehen und Balance Board Standard. Alle Haltungen und Intervalle sind lokal ergänz- und entfernbar.
- Das originale KI-AffAIrs-Logo erscheint dezent neben der Produktkennung „Lotse“.
- Für KW45 wurden drei echte, datensparsam geprüfte Live-Screenshots in der Social-Media-Ablage gesichert; keine Veröffentlichung oder Terminierung wurde ausgelöst.

## Wo liegen die Dinge

- [Projektsteuerung](../../ZIEL.md), [Anforderungsstand](../../AENDERUNGEN.md), [Plan](../../PLAN.md) und [Projektlog](../../ARBEITSLOGBUCH.md).
- Lokale App: `target/release/bundle/macos/Lotse.app`; dazugehörige DMG unter `target/release/bundle/dmg/`.
- [KW45-Lotse-Assets](</Users/kiaffairs/Desktop/KI-Arbeit/31_SocialMedia/KW45/ki-affairs/lotse-live-screenshots/README.md>) und [Social-Media-Handoff](</Users/kiaffairs/Desktop/KI-Arbeit/31_SocialMedia/docs/handoffs/20261028_09_lotse-live-assets.md>).
- [PersonalOS-Aktivität](</Users/kiaffairs/Desktop/personalOS/daily/2026/2026-10-28/activity/01a18345-c858-75d3-9e8d-8a64ba555b4c.md>).

## Verifikation

- `cargo test --workspace`: 30/30 Tests bestanden.
- `npm run tauri build`: Produktions-Bundle und DMG erfolgreich erstellt.
- Sichtprüfung 2026-10-28: Cockpit zeigt Original-Bildmarke und Bürostuhl; Einstellungen zeigen vier Standardhaltungen sowie Hinzufügen/Entfernen.
- Nicht erneut geprüft: A-005 in der spezifischen Konstellation mit acht Ghostty-Tabs sowie Claude Desktop und Codex App. Die aktuell sichtbare Metrik war davon abweichend; deshalb bleibt A-005 umgesetzt, aber nicht geprüft.

## Git-Status

- Branch: `main`; lokaler Stand ist 10 Commits vor `origin/main`.
- Letzter Commit: `62c94bb docs(A-008): Versionsbezug nachtragen`.
- Keine offenen Änderungen, kein Push, kein Release und keine Installation nach `/Applications`.

## Offene Folgeschritte

1. Bei einer passenden Arbeitslage mit acht Ghostty-Tabs sowie Claude Desktop und Codex App A-005 gezielt im Cockpit prüfen, ohne private Titel oder Pfade zu sichern.
2. Nur mit neuem Auftrag: den Lotse-KW45-Beitrag aus den Assets ausarbeiten und dafür Dienstag, Freitag oder eine spätere Woche entscheiden.
3. Für öffentliche Weitergabe später Signierung, Notarisierung und Installationsweg separat planen; nicht Teil dieses Stands.

## Übergabe-Prompt für die nächste Session

> Lies `docs/handoffs/20261028_01_tagesabschluss-lotse.md` und setze bei „Offene Folgeschritte“ fort.
