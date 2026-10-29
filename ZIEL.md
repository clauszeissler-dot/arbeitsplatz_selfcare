# Projektziel: Lotse

- Stand: 2026-10-28
- Owner: Claus Zeißler
- Lebensdauer: dauerhaftes lokales macOS-Produkt
- Outcome: Menschen mit langer KI-gestützter Bildschirmarbeit erhalten auf ihrem Mac konfigurierbare, lokale Erinnerungen mit eindeutigem visuellen Signal und optionalem Ton.
- Ausgangswert: Das bestehende MVP zeigt bereits lokale Hinweis-Karten und speichert Einstellungen lokal; ein vom Nutzer schaltbarer Erinnerungston fehlt.
- Historischer Zielwert A-001 bis A-003: In diesem Arbeitslauf: jede regulär ausgelöste Erinnerung zeigt die bestehende Karte; bei aktiviertem Ton wird zusätzlich ein macOS-Systemton ausgelöst, bei deaktiviertem Ton nicht. Nachweis durch Core-Tests und einen erfolgreichen lokalen Produktions-Build.

## Nutzerablauf und Abnahme

- Nutzerablauf: Beim Arbeiten startet Lotse aus der Menüleiste (auf Wunsch beim Anmelden). Wenn ein konfiguriertes Intervall oder eine Belastungsregel fällig ist, erscheint eine Hinweis-Karte; der Nutzer kann den Ton in den Einstellungen ein- oder ausschalten.
- Sichtbares Ergebnis: Die Einstellungen enthalten den Schalter „Erinnerungston“, und die gebündelte Mac-App enthält die visuelle Erinnerungsfunktion samt optionalem Ton.
- Vereinbarter Zielort: lokaler Projektordner und erzeugtes macOS-App-Bundle unter `target/release/bundle/macos/Lotse.app`.
- Abschlussnachweis am Zielort: `cargo test -p lotse-core` und `npm run tauri build` enden erfolgreich; der Build erzeugt das App-Bundle.
- Bereitstellungsumfang: nur lokaler Build; kein Push, kein Release und keine Installation in Programme ohne weiteren Auftrag.
- Relevanter Fehler- oder Ablehnungsfall: Ist der Ton ausgeschaltet oder der Systemton nicht verfügbar, bleibt die visuelle Hinweis-Karte funktional und die App läuft weiter.
- Erfolgskriterien: Details in [AENDERUNGEN.md](AENDERUNGEN.md).
- Anforderungen: [AENDERUNGEN.md](AENDERUNGEN.md)

## Umfang und Grenzen

- Erlaubte Datenänderungen und Außenwirkungen: lokale Projektdateien und lokale Build-Artefakte; Einstellungen bleiben im lokalen App-Datenordner.
- Leitplanken: keine Cloud-Kommunikation, keine medizinischen Versprechen, keine Veröffentlichung oder Push.
- Nicht gebaut: eigener Klang-Editor, Push-Benachrichtigungen und iOS-/Windows-Versionen; nicht Teil des Auftrags.
- Offene entscheidende Angaben: keine. Der Rhythmus startet bei erkannter Aktivität, ruht bei Pausen und Meetings und behält innerhalb eines Tages seinen Zustand bei App-Neustarts; der Ton ist ein separater Schalter und standardmäßig aus.

## Zielprüfung nach Veröffentlichung

- Nicht beauftragt; lokale Build-Abnahme genügt für diesen Arbeitslauf.

## Zieländerungen

| Datum | Anlass / Anforderungs-ID | Änderung | Entscheidungsquelle |
| --- | --- | --- | --- |
| 2026-10-28 | A-003 | Optionalen, abschaltbaren Ton zum vorhandenen visuellen Hinweis ergänzt. | Nutzerauftrag |
| 2026-10-28 | A-004 | Aktivitätsbasierten Start des vorhandenen Erinnerungsrhythmus verbindlich festgelegt. | Nutzerentscheidung |
| 2026-10-28 | A-006 | Den normalen Bürostuhl als Standardhaltung benannt; nach dem Sitzintervall fordert Lotse zum Haltungswechsel auf. | Nutzerkorrektur |
| 2026-10-28 | A-007 | Bürostuhl und Swopper getrennt geführt; Haltungen und ihre Intervalle sind je Person lokal verwaltbar. | Nutzerkorrektur |
| 2026-10-28 | A-008 | Originales KI-AffAIrs-Logo dezent in die Cockpit-Kopfzeile integriert. | Nutzerwunsch |

Umsetzung: [PLAN.md](PLAN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)


## Arbeitslauf 2026-10-29

Claus kann zeitbezogene Augen- und Pausenübungen direkt mit einem Timer ausführen,
ohne die Zeit selbst mitzählen zu müssen. Ausgangswert: keine Übungstimer, macOS
Glass als Ton und dauerhaft unsichtbar in Dock/⌘Tab. Ziel: Start, sichtbarer
Countdown, einmaliger Abschluss; eigene lokale Klangfamilie mit Probe und
Stummschaltung; Dock/⌘Tab bei geöffnetem Cockpit. Abnahme: A-009 bis A-012.
Zielort bleibt das lokale App-Bundle. Ein nicht startbarer Timer meldet seinen
Fehler sichtbar; die Karte bleibt bedienbar. App-Neustart stellt Timer nicht wieder
her. Klänge werden lokal im App-Datenordner abgelegt; keine Cloud-Funktion ergänzt.

A-013 erweitert die bisherige Grenze ausdrücklich auf lokale Developer-ID-Signatur
und Apple-Notarisierung. Dafür darf das gebaute Paket an Apple gesendet werden;
keine Veröffentlichung im App Store oder im Repository. Das vorhandene Apple-Zugangsprofil `KI-Kennzeichnung-Notarization` wird wiederverwendet;
Freigabe ist durch Apple-Ticket und Gatekeeper-Prüfung nachzuweisen.


## Repository-Download 2026-10-29 (A-014/A-015)

Nutzer autorisiert Upload des fertigen Pakets und README-Verlinkung im bestehenden privaten GitHub-Repository. Release-Dateien aus `target/release/notarized/` unverändert bereitstellen; notwendiger Push der vorhandenen Umsetzung und Dokumentation eingeschlossen. Keine Umstellung auf öffentlich und kein neuer Build.


## Lizenz und Selbstbau 2026-10-29 (A-017/A-018)

Claus erlaubt Nutzung und Weitergabe unveränderter Original-Releases. Er erlaubt
auch Änderungen und Veröffentlichungen unter eigener Identität. Eigenes
Projektmaterial (Code, Dokumentation, komponierte Klänge) unter Apache-2.0;
Originalmarke und Entwickleridentität nicht für eigene Releases mitfreigegeben.
Selbstbau-Anleitung zeigt eigene App-Kennung; veröffentlichte Änderungen tragen
eigenen Namen/eigene Marke und eigene Apple-Signierung. Abnahme: Lizenztexte,
Metadaten, explizite Skript-Konfiguration und Hinweise auf GitHub/Release geprüft.


## Sicherheitsprüfung 2026-10-29 (A-019)

Lotse erhält einen belegten Sicherheitsstatus für den aktuellen Quellstand und die
veröffentlichte Release-Kette. Die Prüfung verändert keine Produktfunktion und
keine GitHub-Einstellung. Abnahme: versiegelter Bericht mit validierten Findings,
konkreter Beschreibung der lokal gespeicherten Daten und Zugriffsrechte sowie
ehrlich benannten Prüflücken. Aktueller Remote-Befund: Repository öffentlich;
frühere historische Zielangaben „privat“ sind keine Aussage über den heutigen
GitHub-Zustand. RustSec-Advisories bleiben offen, bis eine aktuelle Datenquelle
verfügbar bzw. Dependabot aktiviert ist.

## Sicherheitshärtung 2026-10-29 (A-020)

Die drei validierten Findings werden geschlossen und ihre Sicherheitsgrenzen
durch Regressionstests festgehalten. Lokale Aktivitätsdaten bleiben bewusst
unverschlüsselt, werden aber nur mit Benutzerrechten, atomar, geordnet und für
höchstens 60 Tageszusammenfassungen gespeichert. Offizielle Releases erhalten
eine getrennte, manuell freizugebende Signier- und Notarisierungsstufe. Das
öffentliche Solo-Repository verlangt Pull Requests und erfolgreiche Prüfungen;
Force-Push und Löschung von `main` bleiben gesperrt. Abnahme ist der lokal
geprüfte und remote kontrollierte Repository-Stand, nicht ein neuer Release.

## Stabilität und Tagesverlauf 2026-10-29 (A-021 bis A-023)

Lotse soll auch nach vielen ignorierten oder geschlossenen Hinweisen stabil und
sichtbar bleiben. Ein schwarzes Dashboard braucht einen belegten, lokal
diagnostizierbaren Wiederherstellungspfad. Fokus/Aktivität und Pause erhalten
einheitliche semantische Farben. Eine kompakte Tagesübersicht zeigt lokal, wann
erkannte Tasks/Projekte begannen, wie lange sie aktiv waren und wann Pausen
begannen/endeten. Die Übersicht speichert keine Rohpfade, Tabtitel oder
vollständigen Prozessargumente und bleibt auf den begrenzten lokalen Verlauf
beschränkt.
