# Lotse – Anleitung

🇬🇧 [English version](GUIDE.en.md) · [Zurück zur Übersicht](../README.md)

Lotse hilft dir, bei langer Arbeit mit KI frisch zu bleiben. Diese Anleitung zeigt dir in einfachen Schritten,
wie du Lotse einrichtest und benutzt.

## Was du brauchst

- Einen Mac mit macOS 13 oder neuer.
- Das Terminal **Ghostty** in der neuesten Version (mindestens 1.3). Ohne Ghostty läuft Lotse auch, sieht dann aber deine Tabs nicht.

## Installieren

Fertige Downloads gibt es noch nicht. Bis dahin baust du Lotse einmal selbst. Das dauert beim ersten Mal etwa 10 Minuten.

1. Installiere die Werkzeuge, falls du sie noch nicht hast:
   - Xcode Command Line Tools: `xcode-select --install`
   - Rust: siehe [rustup.rs](https://rustup.rs)
   - Node.js 22: siehe [nodejs.org](https://nodejs.org)
2. Lade dieses Repository herunter und öffne den Ordner im Terminal.
3. Gib ein:
   ```sh
   npm install
   npm run tauri build
   ```
4. Die fertige App liegt danach unter `target/release/bundle/macos/Lotse.app`. Zieh sie in deinen Ordner **Programme**.

Sobald es signierte Downloads gibt, lädst du einfach die DMG-Datei herunter und ziehst Lotse in **Programme**.

## Der erste Start

1. Öffne Lotse. Es erscheint ein farbiger Punkt oben in der Menüleiste, aber kein Symbol im Dock.
2. macOS fragt: **„Lotse möchte Ghostty steuern“**. Klicke auf **OK**. Lotse liest nur mit und verändert nichts.
3. Klicke auf den Punkt und dann auf **Cockpit öffnen**.
4. Klicke im Cockpit oben rechts auf das Zahnrad und schalte **„Beim Anmelden starten“** ein.

## Im Alltag

**Die Ampel in der Menüleiste**

- **Grün:** Du arbeitest in einem Tempo, das du den ganzen Tag halten kannst.
- **Gelb:** Noch in Ordnung. Schließ den aktuellen Strang bewusst ab.
- **Rot:** Zu viel gleichzeitig. Park ein Projekt, bevor Fehler passieren.

Die Zahl neben dem Punkt zeigt, wie viele Minuten du schon ohne Pause arbeitest.

**Hinweise beantworten**

Hinweise erscheinen als kleine Karte oben rechts. Du hast zwei Möglichkeiten:
- Klick auf die orange Schaltfläche, wenn du es erledigt hast, zum Beispiel **„Getrunken“** oder **„Stehe“**.
- Klick auf **„Gleich“**, dann kommt der Hinweis später wieder.

Die Karte verschwindet nach 40 Sekunden von selbst. Fährst du mit der Maus darauf, bleibt sie stehen.
Wenn du möchtest, begleitet sie ein kurzer Systemton. Den Schalter **„Erinnerungston abspielen“** findest du in den Einstellungen; er ist zunächst aus, und die sichtbaren Hinweise bleiben immer aktiv.

**Ohne Hinweis eintragen**

Klick auf den Punkt in der Menüleiste:
- **Glas getrunken:** zählt ein Glas.
- **Position:** Swopper, Stehen oder Balance Board.
- **Pause starten / beenden:** für Pausen, in denen du am Rechner bleibst.
- **Hinweise pausieren:** für 30 Minuten, 1 Stunde oder 2 Stunden Ruhe.

**Pausen**

Lotse merkt von selbst, wenn du 10 Minuten nicht an Maus oder Tastatur warst. Das zählt als Pause.
Klappst du den Mac zu, zählt das auch.

**Meetings**

Ist deine Kamera an, oder dein Mikrofon länger als 2 Minuten, kommen keine Hinweise.
Kurzes Diktieren zählt nicht als Meeting. Nach dem Meeting wartet Lotse eine Minute und meldet sich dann.

## Einstellungen

Im Cockpit über das Zahnrad. Du kannst zum Beispiel ändern:
- dein Trinkziel und die Glasgröße
- wie lange du sitzt, stehst oder auf dem Board bist
- wann Pausen fällig sind
- ob Hinweise zusätzlich einen Ton abspielen
- ab wann die Ampel gelb oder rot wird

Klick zum Schluss auf **Speichern**.

## Was Lotse liest und speichert

- **Liest:** die Titel und Ordner deiner Ghostty-Tabs, welche Programme laufen und wie stark sie arbeiten, wie lange du nichts getippt hast und ob irgendwo Kamera oder Mikrofon an sind.
- **Nimmt nichts auf:** Lotse hört nicht mit und sieht kein Kamerabild. Es fragt nur ab, ob ein Gerät an ist.
- **Speichert:** deine Tageswerte und Einstellungen nur auf deinem Mac, im Ordner `~/Library/Application Support/de.kiaffairs.lotse/`.
- **Ohne Internet:** Lotse braucht keine Internetverbindung und schickt nichts weg.

## Wichtige Grenzen

- Lotse erkennt nur Agenten in **Ghostty** und die Apps **ChatGPT** und **Claude Desktop**. Was im Browser läuft, sieht Lotse nicht.
- Ob du sitzt oder stehst, weiß Lotse nur, wenn du es anklickst.
- Ob ein Agent „arbeitet“, schätzt Lotse anhand der Rechenlast. Das kann mal danebenliegen.
- Lässt ein Programm das Mikrofon dauerhaft an, hält Lotse dich für dauernd im Meeting. Schalte dann in den Einstellungen **„Mikrofon zur Meeting-Erkennung nutzen“** aus.

## Hilfe bei Problemen

| Problem | Lösung |
|---|---|
| Im Cockpit steht „Ghostty-Zugriff fehlt“ | Systemeinstellungen → Datenschutz & Sicherheit → Automation → bei Lotse **Ghostty** einschalten. |
| „Ghostty ist zu alt“ | Ghostty auf die neueste Version aktualisieren. |
| Es kommen gar keine Hinweise | Prüfe, ob Hinweise pausiert sind oder ein Meeting angezeigt wird. |
| Werte sollen neu starten | Lotse beenden und die Datei `state.json` im oben genannten Ordner löschen. |
| Alles zurücksetzen | Zusätzlich `settings.json` löschen. |

## Hinweis

Lotse ist eine Erinnerungshilfe. Es ist kein Medizinprodukt und ersetzt keine ärztliche Beratung.
Die Startwerte beruhen auf Studien und Empfehlungen ([Quellen](RECHERCHE.md)), passen aber nicht für jeden Menschen.
Wenn du Schmerzen hast, sprich mit einer Ärztin oder einem Arzt.
