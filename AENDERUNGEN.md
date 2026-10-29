# Anforderungen und Änderungswünsche

Zweck: [ZIEL.md](ZIEL.md) · Umsetzung: [PLAN.md](PLAN.md) · Nachweise: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)

| ID | Eingang / Quelle | Anforderung | Abnahmekriterium | Status | Planschritt | Prüfbeleg / Grund |
| --- | --- | --- | --- | --- | --- | --- |
| A-001 | Nutzer, 2026-10-28 | App arbeitet lokal auf dem Mac. | Tauri-Produktions-Build erzeugt `Lotse.app`; keine neue Netzwerkfunktion ergänzt. | geprüft | 3 | `npm run tauri build`, 2026-10-28: App und DMG erstellt |
| A-002 | Nutzer, 2026-10-28 | Erinnerungen folgen einem einstellbaren Rhythmus und geben ein klares visuelles Signal. | Vorhandene Intervalle und Hinweis-Karte bleiben im Build enthalten; Core-Tests bestehen. | geprüft | 3 | `cargo test -p lotse-core` (25/25), Produktions-Build erfolgreich |
| A-003 | Nutzer, 2026-10-28 | Akustisches Signal kann genutzt und bewusst ausgeschaltet werden. | Einstellungs-Schalter wird lokal gespeichert; aktiv löst er beim Hinweis einen Systemton aus, inaktiv überspringt er ihn. | geprüft | 2–3 | Einstellungen-Serialisierungstest und Produktions-Build erfolgreich; Laufzeit-Ton wird nur bei angezeigter Karte ausgelöst |
| A-004 | Nutzer, 2026-10-28 | Erinnerungsrhythmus arbeitet aktivitätsbasiert. | Ohne erkannte Eingabe startet keine Reminder-Uhr; erst erkannte Aktivität startet sie. Pausen und Meetings erzeugen keine Hinweise. | geprüft | 5 | `inactive_time_does_not_start_the_reminder_rhythm`, `cargo test -p lotse-core` (26/26) |
| A-005 | Nutzer, 2026-10-28 | Lotse erkennt die acht Ghostty-Tabs sowie Claude Desktop und die Codex-Desktop-Komponenten. | Prozessliste wird auch bei großer Ausgabe verarbeitet; Cockpit zeigt die 8 Ghostty-Tabs, Claude Desktop und Codex getrennt. | umgesetzt | 6–8 | Roter/grüner Test für 50.000 Zeilen; Codex-Zuordnungstest; 28 Workspace-Tests grün. Sichtprüfung der neu gestarteten Menüleisten-App offen. |
| A-006 | Nutzer, 2026-10-28 | Der normale Bürostuhl ist die Standardhaltung; nach dem Sitzintervall fordert Lotse ausdrücklich zum Haltungswechsel auf. | Neue und vorhandene Sitz-Zustände erscheinen als „Bürostuhl“. Nach dem Sitzintervall zeigt der Hinweis „Haltung wechseln“ mit Alternativen Stehen und Board. | geprüft | 9–11 | Core-Test grün; Workspace-Tests 29/29 und lokales Produktions-Bundle vorhanden. Sichtprüfung des laufenden Cockpits: Bürostuhl als Standard sichtbar. |
| A-007 | Nutzer, 2026-10-28 | Bürostuhl und Swopper sind getrennte Standardhaltungen. Personen können weitere Haltungen lokal ergänzen oder nicht benötigte entfernen. | Vier Standardhaltungen sind sichtbar. Einstellungen erlauben Namen und Intervall je Haltung, Hinzufügen und Entfernen (mindestens eine Haltung bleibt). Auswahl und Wechselhinweise nutzen die gespeicherte Liste. | geprüft | 12–15 | 30 Workspace-Tests grün; Tauri-Produktions-Bundle und DMG erstellt. Sichtprüfung: Bürostuhl, Swopper, Stehen und Balance Board sowie Hinzufügen/Entfernen sichtbar. |
| A-008 | Nutzer, 2026-10-28 | Das originale KI-AffAIrs-Logo soll dezent statt des reinen Textzugs in der Cockpit-Kopfzeile erscheinen. | Kopfzeile zeigt das vorhandene Original-Logo sowie die Produktkennung „Lotse“; keine neue oder nachgezeichnete Markenvariante. | geprüft | 16–18 | Frontend- und Tauri-Produktions-Build erfolgreich; laufendes Cockpit zeigt die Original-Bildmarke neben „Lotse“. |

| A-009 | Nutzer, 2026-10-29 | Zeitbezogene Übungen per Startknopf mit Countdown und hörbarem Abschluss durchführen. | Augenhinweis startet 20 s; Pausen nutzen konfigurierte Dauer; laufende Timer bleiben sichtbar, sind abbrechbar und werden nicht durch Hinweise ersetzt. Abschluss einmalig sichtbar und hörbar (abschaltbar). | geprüft | Arbeitslauf 2026-10-29, 1/3 | Core-Dauertest, nativer Countdown-Test und Ego Lite: 20 s bis Fertig, stummer Pausentimer/Abbruch, Karte passt in 380 px; 2026-10-29 |
| A-010 | Nutzer, 2026-10-29 | Hochwertige, zusammengehörige eigene Klänge statt Systemton. | Lokal eingebundene, weich ein-/ausgeblendete Stereo-Klänge für Hinweis, Start und Abschluss; Klangprobe und Stummschaltung; kein Glass mehr. | technisch geprüft | Arbeitslauf 2026-10-29, 2/3 | Eigene Stereo-WAVs (-12 dBFS Peak, Ende bei null); Ego-Klangprobe lädt 776204 Byte; genau ein Start-/Abschlussaufruf. Subjektive Klangabnahme durch Nutzer offen |

| A-011 | Nutzer, 2026-10-29 | Arbeitslog während der Umsetzung führen. | Wünsche, Entscheidungen und Prüfstand sind in ARBEITSLOGBUCH.md nachvollziehbar. | geprüft | Arbeitslauf 2026-10-29 | Projektlog fortlaufend ergänzt, globaler Eintrag 2026-10-29 geschrieben |
| A-012 | Nutzer, 2026-10-29 | Geöffnetes Dashboard über Dock und ⌘Tab wiederfinden. | Öffnen aktiviert den regulären macOS-App-Modus; Schließen versteckt das Dashboard und lässt Lotse in der Menüleiste weiterlaufen. | geprüft | Arbeitslauf 2026-10-29 | Native AX-Abfrage: Cockpit geöffnet background-only=false/visible=true; geschlossen background-only=true/visible=false |

| A-013 | Nutzer, 2026-10-29 | App über Apple Developer freigeben. | Lokales Paket mit Developer ID signieren, Apple-Notarisierung erfolgreich, Ticket anheften und Gatekeeper prüfen; fehlende Identität/Zugang konkret ausweisen. | geprüft | Wiederaufnahme A-013 | Apple Accepted für App und DMG; beide Logs ohne Issues. Tickets, Signaturen und Gatekeeper am Bundle, entpackten ZIP und App im schreibgeschützt gemounteten DMG erfolgreich, 2026-10-29. |

| A-014 | Nutzer, 2026-10-29 | Fertige signierte App im bestehenden Repository zum Download anbieten. | GitHub-Release mit geprüftem, notarisiertem DMG/ZIP; heruntergeladene Datei hat denselben Hash wie die freigegebene Quelle. Repository-Sichtbarkeit unverändert. | in Arbeit | Repository-Download 1/3 | vorbereitet |
| A-015 | Nutzer, 2026-10-29 | Download direkt im README verlinken. | Prominenter Direktlink öffnet das hochgeladene DMG; Apple-Silicon/macOS-Voraussetzung und Installation erklärt, Aussage zu fehlenden Downloads entfernt. | in Arbeit | Repository-Download 2/3 | vorbereitet |

## Revisionen und Entscheidungen

| Datum | ID | Was wurde verändert? | Warum / Entscheidungsquelle | Nachfolge-ID, falls vorhanden |
| --- | --- | --- | --- | --- |
| 2026-10-28 | A-002 | „Bestimmter Rhythmus“ als bestehende, pro Hinweisart einstellbare Intervalle konkretisiert. | Im Repository bereits vorhandene Einstellungen für Trinken, Haltung, Pausen und Mindestabstand; keine zusätzliche Nutzerentscheidung erforderlich. | — |
| 2026-10-28 | A-004 | Aktivitätsbasierte Auslegung von „ab Start“ verbindlich entschieden. | Nutzerentscheidung nach Erläuterung der Alternativen. | — |
| 2026-10-28 | A-006 | Der bisherige Name „Swopper“ wird durch „Bürostuhl“ ersetzt; die bestehende Sitz-Zeit bleibt der Standard und führt nach Ablauf zum Wechselhinweis. | Nutzerkorrektur: normaler Bürostuhl ist die tatsächliche Standardhaltung. | — |
| 2026-10-28 | A-007 | A-006 wird fachlich erweitert: Swopper bleibt eine eigene Haltung; Haltungen werden pro Person lokal verwaltbar. | Nutzerkorrektur: Tool soll für andere Mac-Nutzer mit eigenen Haltungen teilbar sein. | — |
| 2026-10-28 | A-008 | Reinen Textzug in der Kopfzeile durch das vorhandene Original-Logo ergänzen. | Nutzerwunsch |

## Letzter Abschlussabgleich

- Stand: 2026-10-28
- Abgeglichene IDs: A-001 bis A-004
- Geprüft erfüllt: A-001 bis A-004; Belege in den Tabellenzeilen und [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md).
- Nicht erfüllt: A-005 ist umgesetzt, aber die Sichtprüfung des neu gestarteten Cockpits steht offen.
- Nicht erfüllt: A-006 ist umgesetzt, aber die Sichtprüfung des neu gestarteten Cockpits steht offen.
- Nicht erfüllt: A-007 ist umgesetzt, aber die Sichtprüfung des neu gestarteten Cockpits steht offen.
- Nicht erfüllt: A-008 ist umgesetzt, aber die Sichtprüfung des neu gestarteten Cockpits steht offen.
- Neue Wünsche seit dem letzten Abgleich vollständig erfasst: ja (A-008 in Arbeit)


## Abschlussabgleich 2026-10-29

- A-009, A-011 und A-012 geprüft; A-010 technisch geprüft, persönliche Klangbewertung durch Claus offen.
- A-013: signierte lokale App und DMG vorhanden. Apple-Notarisierung ist durch fehlendes Schlüsselbundprofil blockiert; nächster Schritt: Accountinhaber richtet `lotse` lokal ein, dann `scripts/notarize.sh lotse` ausführen und Apples Ergebnis prüfen.
- Frühere IDs: A-001–A-004 und A-006–A-008 behalten ihre vorhandenen Prüfbelege; A-003 verwendet ab jetzt gemäß A-010 eigene Klänge statt Glass. A-005 bleibt mit dem früher dokumentierten separaten Sichtprüfpunkt offen und gehört nicht zu diesem Änderungsauftrag.

- Revision A-013, 2026-10-29: Aus fehlendem Profil `lotse` war fälschlich auf fehlenden Apple-Zugang geschlossen worden. Das bestehende, projektübergreifend verwendbare Profil `KI-Kennzeichnung-Notarization` funktioniert. Kein neues Passwort nötig.

## Aktualisierter Abschluss A-013, 2026-10-29

Apple-Notarisierung abgeschlossen; früherer Blocker aufgehoben. App/ZIP/DMG geprüft. Fertige Weitergabe-Artefakte unter `target/release/notarized/`. Kein Passwort-Reset oder neues Profil nötig; keine Veröffentlichung, kein Push.
