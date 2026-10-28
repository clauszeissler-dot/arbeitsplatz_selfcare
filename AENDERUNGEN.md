# Anforderungen und Änderungswünsche

Zweck: [ZIEL.md](ZIEL.md) · Umsetzung: [PLAN.md](PLAN.md) · Nachweise: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

| ID | Eingang / Quelle | Anforderung | Abnahmekriterium | Status | Planschritt | Prüfbeleg / Grund |
| --- | --- | --- | --- | --- | --- | --- |
| A-001 | Nutzer, 2026-10-28 | App arbeitet lokal auf dem Mac. | Tauri-Produktions-Build erzeugt `Lotse.app`; keine neue Netzwerkfunktion ergänzt. | geprüft | 3 | `npm run tauri build`, 2026-10-28: App und DMG erstellt |
| A-002 | Nutzer, 2026-10-28 | Erinnerungen folgen einem einstellbaren Rhythmus und geben ein klares visuelles Signal. | Vorhandene Intervalle und Hinweis-Karte bleiben im Build enthalten; Core-Tests bestehen. | geprüft | 3 | `cargo test -p lotse-core` (25/25), Produktions-Build erfolgreich |
| A-003 | Nutzer, 2026-10-28 | Akustisches Signal kann genutzt und bewusst ausgeschaltet werden. | Einstellungs-Schalter wird lokal gespeichert; aktiv löst er beim Hinweis einen Systemton aus, inaktiv überspringt er ihn. | geprüft | 2–3 | Einstellungen-Serialisierungstest und Produktions-Build erfolgreich; Laufzeit-Ton wird nur bei angezeigter Karte ausgelöst |

## Revisionen und Entscheidungen

| Datum | ID | Was wurde verändert? | Warum / Entscheidungsquelle | Nachfolge-ID, falls vorhanden |
| --- | --- | --- | --- | --- |
| 2026-10-28 | A-002 | „Bestimmter Rhythmus“ als bestehende, pro Hinweisart einstellbare Intervalle konkretisiert. | Im Repository bereits vorhandene Einstellungen für Trinken, Haltung, Pausen und Mindestabstand; keine zusätzliche Nutzerentscheidung erforderlich. | — |

## Letzter Abschlussabgleich

- Stand: 2026-10-28
- Abgeglichene IDs: A-001 bis A-003
- Geprüft erfüllt: A-001 bis A-003; Belege in den Tabellenzeilen und [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md).
- Nicht erfüllt: keine. Offene Produktentscheidung außerhalb der Abnahme: Soll der aktivitätsbasierte Rhythmus bei jedem App-Start neu beginnen oder seinen Tageszustand behalten?
- Neue Wünsche seit dem letzten Abgleich vollständig erfasst: ja
