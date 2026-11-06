# Änderungsprotokoll

Format nach „Keep a Changelog“, Versionen nach SemVer (`X.Y.Z`). Zu jedem Release gehört ein Abschnitt
mit Inhalt; er wird als Release-Text verwendet. Versionen ändert ausschließlich
`scripts/set-version.sh`.

## [Unveröffentlicht]

## [0.3.0] – noch nicht veröffentlicht

### Neu

- Fokus, Pause und Haltung zeigen Startzeit und eine laufende Uhr; laufende Aufgaben und Pausen zählen live.
- „Tag beenden“ (Kopfzeile, Menüleiste, Hinweis „Arbeitstag ist voll“) speichert ein Tagesprotokoll und beendet Lotse. Am selben Tag geht es beim Neustart weiter.
- Tagesprotokolle (Aufgaben, Pausen, Zeitraum) sind in „Letzte Tage“ aufklappbar.
- Sichere Updates aus den offiziellen GitHub Releases (signiert, mit Zustimmung vor der Installation).
- Die Einstellungen zeigen Version und Build-Art (Release oder lokaler Build).

### Behoben

- Universal-Build lädt die eingebettete Oberfläche statt eines Entwicklungsservers (schwarzes Cockpit).

### Hinweis zum Wechsel

- Version 0.1.0 hat keinen Updater. Der Wechsel auf 0.3.0 erfolgt einmalig manuell per DMG; danach bietet Lotse Updates selbst an.

## [0.1.0] – 2026-10-29

- Erster signierter und von Apple notarisierter Release (Apple Silicon): Menüleisten-Ampel, Erinnerungen für Pausen, Trinken, Haltung und Augen, Kurz-Timer, eigene Klänge, Zeitübersicht.
