# Anforderungen und Änderungswünsche

Zweck: [ZIEL.md](ZIEL.md) · Umsetzung: [PLAN.md](PLAN.md) · Nachweise: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

| ID | Eingang / Quelle | Anforderung | Abnahmekriterium | Status | Planschritt | Prüfbeleg / Grund |
| --- | --- | --- | --- | --- | --- | --- |
| A-001 | Nutzer, 2026-10-28 | App arbeitet lokal auf dem Mac. | Tauri-Produktions-Build erzeugt `Lotse.app`; keine neue Netzwerkfunktion ergänzt. | geprüft | 3 | `npm run tauri build`, 2026-10-28: App und DMG erstellt |
| A-002 | Nutzer, 2026-10-28 | Erinnerungen folgen einem einstellbaren Rhythmus und geben ein klares visuelles Signal. | Vorhandene Intervalle und Hinweis-Karte bleiben im Build enthalten; Core-Tests bestehen. | geprüft | 3 | `cargo test -p lotse-core` (25/25), Produktions-Build erfolgreich |
| A-003 | Nutzer, 2026-10-28 | Akustisches Signal kann genutzt und bewusst ausgeschaltet werden. | Einstellungs-Schalter wird lokal gespeichert; aktiv löst er beim Hinweis einen Systemton aus, inaktiv überspringt er ihn. | geprüft | 2–3 | Einstellungen-Serialisierungstest und Produktions-Build erfolgreich; Laufzeit-Ton wird nur bei angezeigter Karte ausgelöst |
| A-004 | Nutzer, 2026-10-28 | Erinnerungsrhythmus arbeitet aktivitätsbasiert. | Ohne erkannte Eingabe startet keine Reminder-Uhr; erst erkannte Aktivität startet sie. Pausen und Meetings erzeugen keine Hinweise. | geprüft | 5 | `inactive_time_does_not_start_the_reminder_rhythm`, `cargo test -p lotse-core` (26/26) |

## Revisionen und Entscheidungen

| Datum | ID | Was wurde verändert? | Warum / Entscheidungsquelle | Nachfolge-ID, falls vorhanden |
| --- | --- | --- | --- | --- |
| 2026-10-28 | A-002 | „Bestimmter Rhythmus“ als bestehende, pro Hinweisart einstellbare Intervalle konkretisiert. | Im Repository bereits vorhandene Einstellungen für Trinken, Haltung, Pausen und Mindestabstand; keine zusätzliche Nutzerentscheidung erforderlich. | — |
| 2026-10-28 | A-004 | Aktivitätsbasierte Auslegung von „ab Start“ verbindlich entschieden. | Nutzerentscheidung nach Erläuterung der Alternativen. | — |

## Letzter Abschlussabgleich

- Stand: 2026-10-28
- Abgeglichene IDs: A-001 bis A-004
- Geprüft erfüllt: A-001 bis A-004; Belege in den Tabellenzeilen und [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md).
- Nicht erfüllt: keine.
- Neue Wünsche seit dem letzten Abgleich vollständig erfasst: ja
