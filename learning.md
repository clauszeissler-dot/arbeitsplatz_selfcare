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


### L-003 – Eigene Klanggestaltung und auffindbares Cockpit

- Datum / Status: 2026-10-29 · beobachtet; konkrete Änderungen durch Nutzer beauftragt.
- Quelle: Nutzerfeedback zu unpassendem Systemklang und fehlendem Fenster in Dock/⌘Tab.
- Geltung: Lotse-Klanggestaltung und macOS-Fensterverhalten.
- Beobachtung: Glass erfüllt die bloße Signalfunktion, aber nicht den gewünschten Klangcharakter. Dauerhaftes Accessory blendet auch ein geöffnetes Cockpit aus der App-Auswahl aus.
- Konsequenz / zuständige Umsetzung: A-010 liefert eine eigene lokale Klangfamilie mit Klangprobe; A-012 schaltet beim Öffnen auf Regular und beim Schließen auf Accessory. Beide Entscheidungen sind konkrete Produktanforderungen, keine allgemeine Designregel.
- Prüfpunkt: Ego-Lite-Klangprobe lädt eigene WAV-Datei; native App ist bei geöffnetem Cockpit nicht mehr background-only und nach Schließen wieder background-only.
- Wirkung: Technische Wirkung am 2026-10-29 belegt. Subjektive Bewertung der neuen Klänge durch Claus bleibt offen; aus erfolgreichem Playback folgt keine Geschmacksabnahme.
