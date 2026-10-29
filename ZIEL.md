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
