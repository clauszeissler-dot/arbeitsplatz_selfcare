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


### L-007 – Tauri-App-Prüfungen auf dem Zielbetriebssystem ausführen

- Datum / Status: 2026-10-29 · bestätigt/aktiv durch fehlgeschlagenen und korrigierten PR-Check.
- Quelle: GitHub-Actions-Lauf `40859104108`; Ubuntu scheiterte beim Workspace-Build an fehlendem `glib-2.0`, obwohl die plattformneutrale Core-Crate Linux unterstützt.
- Geltung: CI für Lotse und andere Tauri-macOS-Crates in diesem Repository.
- Beobachtung: `cargo test --workspace` zieht auf Linux die Tauri-/GTK-Abhängigkeiten der macOS-App ein. Das ist keine Produktregression und macht den Linux-Runner unnötig zum Ersatz für den Zielplattform-Build.
- Konsequenz / zuständige Regel: Ubuntu prüft `lotse-core` und RustSec; vollständige Workspace-Tests, Clippy und Universal-App-Build laufen im macOS-Job. macOS-Cross-Targets werden nur im macOS-Job installiert.
- Prüfpunkt: PR muss sowohl `Kernlogik und Security` als auch `Mac-App bauen (unsigniert)` bestehen.
- Wirkung: Korrigierter Lauf `40859430217` grün; Kernjob 2:51 Minuten, macOS-Job 9:18 Minuten. Nächste passende CI-Änderung muss diese Plattformgrenze erhalten.


### L-008 – Bundle-Executable niemals aus dem Produktnamen ableiten

- Datum / Status: 2026-10-29 · bestätigt/aktiv durch fehlgeschlagenen strikten Signaturtest.
- Quelle: A-020-Nachkontrolle; `Info.plist` enthielt `CFBundleExecutable=lotse`, während `build-universal.sh` `Contents/MacOS/Lotse` verwendete.
- Geltung: Universal-Builds und Signierung von Lotse.
- Beobachtung: Das standardmäßig case-insensitive macOS-Dateisystem kaschiert abweichende Groß-/Kleinschreibung. `lipo` kann dadurch ein scheinbar funktionierendes Bundle erzeugen, dessen Code-Signatur die Info.plist-Bindung ablehnt.
- Konsequenz / zuständige Regel: `scripts/build-universal.sh` liest den Namen aus beiden Plists, verlangt identische sichere Werte und verwendet ihn unverändert. Kein Ableiten aus `productName` oder App-Verzeichnis.
- Prüfpunkt: statischer Regressionstest plus realer Universal-Build, Developer-ID-Signatur und `codesign --verify --deep --strict`.
- Wirkung: Folge-PR #10; roter Test vor Korrektur, danach `x86_64 arm64` und strikte Signaturprüfung grün. Bei Änderungen am Bundling erneut prüfen.


### L-009 – Release-WebViews müssen eingebettete Ressourcen laden

- Datum / Status: 2026-10-29 · bestätigt/aktiv durch Nutzerbeobachtung und Codebefund.
- Quelle: Nutzer meldet weiterlaufende Klänge bei fehlenden Hinweisen und schwarzem Dashboard; Neustart heilte den Zustand.
- Geltung: Dashboard- und Toast-Initialisierung von Lotse.
- Beobachtung: Für den Prozess selbst gab es keinen Crashbericht. Die erste plausible Diagnose zu flüchtigen Events war unvollständig: Nach der Installation reproduzierte der Nutzer den Ausfall erneut. Beide WebViews protokollierten `Failed provisional load`, Fehler `-1004`; die Diagnose-Datei fehlte, weil JavaScript nie startete. Der neue Universal-Build war ohne Tauri `custom-protocol` kompiliert und lud `devUrl`, während Engine und Klang nativ weiterliefen.
- Konsequenz / zuständige Umsetzung: A-021/A-024. Event-/Start-Recovery bleibt als zusätzliche Härtung. Verbindlich ist außerdem: jeder per plain Cargo erzeugte Tauri-Release aktiviert `custom-protocol`; der Universal-Build hat dafür einen statischen Regressionstest. Ein schwarzes Fenster ohne UI-Diagnose zuerst auf Dokumentladefehler prüfen.
- Prüfpunkt: Dev-Server beenden, final gebündelte App neu starten, im Unified Log `-1004` ausschließen und feste UI-Überschriften im nativen WebView nachweisen. Buildtest muss beide Architekturen erfassen.
- Wirkung: Roter/grüner Buildtest; korrigiertes Universal-Bundle enthält eingebettete Ressourcen, final notarisierten Prozess ohne Dev-Server mit Cockpit/Zeitleiste nachgewiesen. Veröffentlichter 0.1.0-Release separat ohne Fehler gestartet. Bei nächstem Universal-Release erneut prüfen.
