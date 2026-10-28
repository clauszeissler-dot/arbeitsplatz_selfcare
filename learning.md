# Learnings: Lotse

Owner: Claus Zeißler · Geltungsbereich: Lotse als lokale macOS-Erinnerungs-App

Vor Arbeit und Wiederaufnahme passende aktive Learnings lesen. Die gemeinsamen Regeln stehen in [`../learning.md`](../learning.md).

## Aktive Einträge

### L-001 – Tatsächliche Standardhaltung eindeutig benennen

- Datum / Status: 2026-10-28 · ersetzt durch L-002.
- Quelle: Nutzerkorrektur zur Haltungsauswahl.
- Geltung: Haltungsanzeige und Erinnerungstexte in Lotse.
- Beobachtung: Die technische Standardhaltung `sitting` wurde als „Swopper“ gezeigt, obwohl der gewöhnliche Bürostuhl die tatsächliche Ausgangshaltung ist.
- Konsequenz: Die alleinige Umbenennung war zu eng; die umfassendere Regel steht in L-002.
- Zuständige Umsetzung: historisch A-006; Nachfolge A-007.

### L-002 – Reale Optionen erhalten und individuell verwaltbar machen

- Datum / Status: 2026-10-28 · bestätigt/aktiv.
- Quelle: Nutzerkorrektur, dass Bürostuhl den Swopper ergänzen muss und das Tool für andere Mac-Nutzer teilbar sein soll.
- Geltung: Persönliche Haltungswahl und wechselbasierte Erinnerungen in Lotse.
- Beobachtung: Eine tatsächliche Standardhaltung zu ergänzen darf eine vorhandene, weiterhin nützliche Haltung nicht ersetzen. Ein starres Enum erlaubt zudem keine individuellen Haltungen.
- Konsequenz: Haltungen als lokal gespeicherte Liste führen; die vier Standards Bürostuhl, Swopper, Stehen und Balance Board mitliefern; Hinzufügen, Intervall und Entfernen im Cockpit anbieten.
- Zuständige Umsetzung: A-007, `Settings.postures`, Cockpit und Haltungs-Engine.
- Prüfpunkt: Core-Test prüft Standards, eigene Haltung, Wechselhinweis und Entfernen; laufende Einstellungen zeigen die vier Standards sowie Hinzufügen und Entfernen.
- Wirksamkeitsprüfung: Am 2026-10-28 im laufenden Cockpit und den Einstellungen bestätigt.

Aktuelle Arbeit: [PLAN.md](PLAN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)
