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


### L-004 – Vor neuem Apple-Zugang bestehende Einrichtung ermitteln

- Datum / Status: 2026-10-29 · bestätigt/aktiv durch Nutzerkorrektur und erfolgreiche Apple-Authentifizierung.
- Quelle: Nutzer erinnert an vorherige Einrichtung; TimeAffAIrs-Handoff `docs/handoffs/20260922_01_apple_signing_setup.md`, 2026-09-22.
- Geltung: Lokale Apple-Signierung/Notarisierung für Lotse.
- Beobachtung: Ein nicht vorhandenes profilspezifisches `lotse` wurde fälschlich als fehlender Apple-Zugang gewertet. Ein gültiges bestehendes Profil war vorhanden.
- Konsequenz: Bestehende Release-/Handoff-Dokumentation prüfen und vorhandenes Notarisierungsprofil referenzieren, bevor neue Credentials oder Passwort-Reset verlangt werden. Keine Secrets auslesen.
- Zuständige Regel / Umsetzung: `docs/RELEASE.md`, Profil für offizielle Owner-Releases ist `KI-Kennzeichnung-Notarization`; seit A-018 werden Identität und Profil ausdrücklich übergeben, damit Forks keine fremde Identität als Default übernehmen.
- Prüfpunkt: Authentifizierter History-Aufruf und anschließende tatsächliche Einreichung.
- Wirkung: Profil erfolgreich authentifiziert; App und DMG anschließend Accepted, Tickets und Gatekeeper erfolgreich. Dieser Fall ohne neue Credentials gelöst. Wirkung bei nächster Wiederaufnahme noch offen.


### L-005 – Lizenz und Entwickleridentität bei Selbstbau klar trennen

- Datum / Status: 2026-10-29 · bestätigt/aktiv durch Nutzerauftrag und Entscheidung zu eigenen Veröffentlichungen.
- Quelle: Nutzer bemängelt fehlende Lizenz; erlaubt unveränderte Originalweitergabe und Änderungen unter eigener Identität.
- Geltung: Lotse-Repository, Selbstbau und Release-Downloads.
- Beobachtung: README bot Selbstbau an, Cargo nannte UNLICENSED; kein Lizenztext. Notarisierungsskript verwendete feste Originalidentität und Profil-Default.
- Konsequenz / zuständige Regel: LICENSE/NOTICE/BRANDING.md sowie docs/RELEASE.md. Code-Lizenz, Originalweitergabe und Marken-/Entwickleridentität unterscheiden. Eigene Builds mit eigener Kennung; veröffentlichte Änderungen unter eigener Marke und eigenen Apple-Zugängen. Skript verlangt explizite Identität und Profil.
- Prüfpunkt: Lizenz-Metadaten konsistent, Originalpakete unverändert, Lizenzhinweise am Download, fehlende Signierkonfiguration stoppt vor externen Aktionen.
- Wirkung: Drei Konfigurationstests zuerst rot und danach grün; weitere Release-/Remote-Belege werden im Projektlog festgehalten. Nächste passende Wiederaufnahme noch offen.


### L-006 – CLI-Host und Daybreak-Advisory getrennt belegen

- Datum / Status: 2026-10-29 · bestätigt/aktiv durch Nutzerkorrektur.
- Quelle: Nutzer stellt klar, dass die Sitzung in Codex CLI läuft und seine Daybreak-Freigabe Ende September verlängert wurde.
- Geltung: Codex-Security-Aufrufe für Lotse.
- Beobachtung: Aus einem Deep-Scan-Berechtigungsfehler wurde fälschlich auf Codex Desktop geschlossen. Zusätzlich meldete der Advisory-Endpunkt `not_granted`, obwohl der Nutzer den tatsächlichen Zugang bestätigt.
- Konsequenz: Host aus dem effektiven Sitzungskontext bestimmen und den CLI-geeigneten Standardscan verwenden. Advisory-Status nicht als Sperre oder als stärkeren Beleg als die bestätigte Kontofreigabe behandeln; Widerspruch offen dokumentieren.
- Zuständige Umsetzung: A-019 und Codex-Security-Standardscan.
- Prüfpunkt: Scan läuft im CLI-Workflow bis zum versiegelten Bericht; Daybreak-Widerspruch wird nicht als fehlender Nutzerzugang ausgegeben.
- Wirkung: Standardscan erfolgreich abgeschlossen und Bericht erzeugt. Ursache der falschen Advisory-Zuordnung bleibt extern ungeklärt.
