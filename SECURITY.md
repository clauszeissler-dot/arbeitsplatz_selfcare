# Sicherheitsrichtlinie

## Unterstützte Versionen

Sicherheitskorrekturen gelten für den aktuellen Stand auf `main` und den jeweils
neuesten offiziellen, signierten Lotse-Release. Ältere Releases werden nicht
separat gepflegt.

## Schwachstellen melden

Bitte keine noch nicht behobene Schwachstelle öffentlich als Issue einstellen.
Nutze stattdessen GitHubs private Schwachstellenmeldung unter **Security →
Advisories → Report a vulnerability**. Beschreibe betroffene Version,
reproduzierbare Schritte, erwartetes Verhalten und mögliche Auswirkungen. Keine
echten Passwörter, Schlüssel, privaten Tabtitel oder Arbeitsverzeichnisse
mitschicken.

## Schutzgrenzen

Lotse verarbeitet Aktivitätsdaten lokal. Einstellungen, zusammengefasste
Verlaufsdaten und die auf 50 Einträge begrenzte heutige Aufgaben-/Pausenzeitleiste liegen als unverschlüsseltes JSON im benutzerspezifischen
macOS-App-Datenordner. Ordner und Dateien werden auf den angemeldeten Benutzer
beschränkt; Prozesse desselben Benutzerkontos bleiben außerhalb dieser
Schutzgrenze. Die Zeitleiste enthält nur aus dem letzten Pfadsegment abgeleitete
Anzeigenamen und Zeitwerte. Tabtitel, Arbeitsverzeichnisse und vollständige Prozessargumente
werden nicht dauerhaft gespeichert und nicht an einen Server übertragen.

Ab Version 0.2.0 ruft Lotse zur Updateprüfung ausschließlich die statischen
Release-Metadaten und bei ausdrücklicher Installation das Updatepaket aus dem
offiziellen GitHub-Repository ab. Dabei werden keine lokalen Arbeitsdaten
übertragen. Der Endpunkt ist fest auf HTTPS gebunden; jedes Paket wird vor der
Installation mit dem in der App enthaltenen öffentlichen Schlüssel geprüft. Die
Signatur bindet zusätzlich die angekündigte Version, sodass ein altes gültig
signiertes Paket nicht als neuere Version ausgegeben werden kann.

Offizielle Downloads werden mit Developer ID signiert, von Apple notarisiert
und mit SHA-256-Prüfsummen veröffentlicht. Updater-Pakete erhalten davon getrennt
eine Tauri-Signatur; der private Update-Schlüssel liegt nur im lokalen
Schlüsselbund und im manuell freizugebenden GitHub-Release-Environment.
Änderungen und Forks müssen gemäß
[BRANDING.md](BRANDING.md) eine eigene App-Kennung, Marke und Entwickleridentität
verwenden.
