# Projektziel: Lotse

- Stand: 2026-10-28
- Owner: Claus Zeißler
- Lebensdauer: dauerhaftes lokales macOS-Produkt
- Outcome: Menschen mit langer KI-gestützter Bildschirmarbeit erhalten auf ihrem Mac konfigurierbare, lokale Erinnerungen mit eindeutigem visuellen Signal und optionalem Ton.
- Ausgangswert: Das bestehende MVP zeigt bereits lokale Hinweis-Karten und speichert Einstellungen lokal; ein vom Nutzer schaltbarer Erinnerungston fehlt.
- Zielwert und Zeitraum: In diesem Arbeitslauf: jede regulär ausgelöste Erinnerung zeigt die bestehende Karte; bei aktiviertem Ton wird zusätzlich ein macOS-Systemton ausgelöst, bei deaktiviertem Ton nicht. Nachweis durch Core-Tests und einen erfolgreichen lokalen Produktions-Build.

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

Umsetzung: [PLAN.md](PLAN.md) · Verlauf: [ARBEITSLOGBUCH.md](ARBEITSLOGBUCH.md)
