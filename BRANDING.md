# Nutzung, Weitergabe und eigene Versionen

Rechteinhaber des eigenen Projektmaterials: Claus Zeißler / KI AffAIrs.

## Quellcode und eigene Inhalte

Der eigene Quellcode, die Dokumentation und die eigens für Lotse komponierten
Klänge stehen unter **Apache-2.0**, siehe [LICENSE](LICENSE) und [NOTICE](NOTICE).
Du darfst sie nutzen, selbst bauen, verändern und unter den Bedingungen dieser
Lizenz weitergeben, auch als eigene veröffentlichte Version. Die Lizenz verlangt
unter anderem die Weitergabe des Lizenztexts, den Erhalt zutreffender
Urheberhinweise und eine Kennzeichnung geänderter Dateien.

Fremde Bibliotheken und Schriftarten behalten ihre jeweiligen Lizenzen. Die
Projektlizenz lizenziert fremdes Material nicht neu. Originale Marken-, Logo- und
Produktgrafiken sind von der Apache-Lizenzierung des eigenen Codes ausgenommen.
Dazu gehören insbesondere die KI-AffAIrs-Bildmarke im Markenbanner, die daraus
verwendeten Ausschnitte sowie die originalen Produkt-/App-Icons. Markenrechte
werden gemäß Abschnitt 6 der Apache-Lizenz nicht eingeräumt.

## Unveränderte offizielle App

Du darfst die von Claus Zeißler / KI AffAIrs veröffentlichten **unveränderten
Originalpakete** verwenden, kopieren und weitergeben, einschließlich der darin
enthaltenen Originalmarke und der vorhandenen Developer-ID-Signatur. Diese
zusätzliche Erlaubnis zur unveränderten Darstellung der Marke gilt für das
Originalpaket; sie erlaubt keine Nutzung als Marke eines eigenen Produkts.

Diese Erlaubnis gilt ausdrücklich auch für den bereits veröffentlichten Release
**`lotse-0.1.0`**. Gib bei dessen Weitergabe `LICENSE`, `NOTICE` und diese Datei mit,
soweit sie dem Paket separat beiliegen. Verweise nach Möglichkeit auf den
[Original-Release](https://github.com/clauszeissler-dot/arbeitsplatz_selfcare/releases/tag/lotse-0.1.0).
Die Dateiprüfsummen im Release erlauben den Vergleich mit dem Original.

Die unveränderte Weitergabe überträgt keine Verfügungsrechte über die
Entwickleridentität, Zertifikate oder Schlüssel. Du erhältst keine Berechtigung,
weitere Pakete mit der Entwickleridentität von Claus Zeißler zu signieren oder
zur Notarisierung unter seinem Account einzureichen.

## Eigene Builds und veränderte Veröffentlichungen

Für selbst gebaute Fassungen verwendest du eine **eigene App-Kennung
(Bundle-Identifier)**, zum Beispiel `de.deinname.pausenhelfer`. Dadurch bleibt auch
der lokale App-Datenordner von dem der Original-App getrennt. Die Originalkennung
`de.kiaffairs.lotse` kennzeichnet die offizielle Lotse-App und ist keine
Entwicklerberechtigung für Forks.

Wenn du eine veränderte Fassung veröffentlichst:

- verwende einen **eigenen Produktnamen, eigene Marken-/App-Grafiken und eine
  eigene App-Kennung**; kennzeichne die Fassung als unabhängig und verändert;
- behalte die nach Apache-2.0 erforderlichen Urheber- und Herkunftshinweise bei.
  Eine sachliche Angabe wie „Basiert auf Lotse von KI AffAIrs“ ist zulässig;
- nutze für Signierung und Apple-Notarisierung ausschließlich **deine eigene
  Entwickleridentität und deine eigenen Zugänge**;
- stelle die Fassung nicht als von Claus Zeißler / KI AffAIrs signiert,
  veröffentlicht, unterstützt oder freigegeben dar.

Das Kopieren eines öffentlichen Namens, einer Team-ID oder einer App-Kennung
gewährt keine Rechte an der Entwickleridentität. Zertifikate, private Schlüssel,
Schlüsselbund-Zugänge und Notarisierungstickets sind keine zum Erstellen eigener
Releases erteilten Berechtigungen. Die vorhandene Signatur und das Apple-Ticket
belegen den geprüften Originalstand; sie sind keine Freigabe für geänderte Builds.

Diese Datei beschreibt die gesonderte Marken-/Identitätsnutzung. Sie ändert den
unveränderten Apache-2.0-Lizenztext und dessen Rechte am lizenzierten Code nicht.

Anleitungen: [Selbst bauen](docs/ANLEITUNG.md#alternativ-selbst-bauen) ·
[Eigene signierte Veröffentlichung](docs/RELEASE.md).

Quellen: [Apache-2.0, insbesondere Abschnitte 4 und 6](https://www.apache.org/licenses/LICENSE-2.0) ·
[Apple: Understanding the Code Signature](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/AboutCS/AboutCS.html).
