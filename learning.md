# Learnings: Lotse

Owner: Claus Zeißler · Geltungsbereich: Lotse als lokale macOS-Erinnerungs-App

Vor Arbeit und Wiederaufnahme passende aktive Learnings lesen. Die gemeinsamen Regeln stehen in [`../learning.md`](../learning.md).

## Aktive Einträge

### L-001 – Tatsächliche Standardhaltung eindeutig benennen

- Datum / Status: 2026-10-28 · beobachtet.
- Quelle: Nutzerkorrektur zur Haltungsauswahl.
- Geltung: Haltungsanzeige und Erinnerungstexte in Lotse.
- Beobachtung: Die technische Standardhaltung `sitting` wurde als „Swopper“ gezeigt, obwohl der gewöhnliche Bürostuhl die tatsächliche Ausgangshaltung ist.
- Konsequenz: Die persistierte Kennung bleibt für vorhandene lokale Zustände kompatibel; ihre sichtbare Bezeichnung ist „Bürostuhl“, und der Hinweis danach fordert explizit zum Haltungswechsel auf.
- Zuständige Umsetzung: `Posture::label`, Menü-, Cockpit- und Hint-Texte in A-006.
- Prüfpunkt: Der Core-Test bestätigt Standardbezeichnung, Sitzintervall und Wechselaktionen; die sichtbare Cockpit-Prüfung steht noch aus.
- Nächste Wirksamkeitsprüfung: Beim Öffnen des Cockpits der neu gebauten App.

Aktuelle Arbeit: [PLAN.md](PLAN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)
