Habe gemacht Docker !!!


## Dokumentation

Für diese Aufgabe wurde der Ordner `apis` aus dem Repository in einen neuen Ordner `solution_Niklas` kopiert. Anschließend wurde für jede enthaltene API ein eigenes Dockerfile erstellt, damit jede Anwendung in einem separaten Container ausgeführt werden kann. Die Dockerfiles wurden jeweils an die verwendete Programmiersprache angepasst, also für Go, JavaScript und Python. Dabei wurde darauf geachtet, die passenden Basis-Images zu verwenden und die benötigten Abhängigkeiten korrekt zu installieren.

Zusätzlich wurde im Hauptordner der Lösung eine `docker-compose.yml` Datei angelegt. Diese Datei dient dazu, alle APIs gemeinsam mit nur einem Befehl zu starten. Dafür verweist jeder Service in der Compose-Datei auf den jeweiligen Unterordner der API. So wird für jede Anwendung ein eigenes Image gebaut und der passende Port nach außen freigegeben. Dadurch können alle Endpunkte parallel getestet werden.

Vor dem Start wurde geprüft, ob die jeweiligen Projektordner die nötigen Dateien enthalten. Bei der Go-API musste zuerst ein Go-Modul mit `go mod init` erstellt werden, da anfangs keine `go.mod` vorhanden war. Bei den anderen APIs wurden die vorhandenen Projektdateien wie `package.json` oder `requirements.txt` entsprechend genutzt.

Zum Starten aller Container wird im Hauptverzeichnis der Lösung der Befehl `docker compose up --build` ausgeführt. Danach können die einzelnen APIs über die in der Compose-Datei definierten Ports im Browser oder mit einem Testtool aufgerufen werden. So lässt sich prüfen, ob alle Container korrekt gestartet wurden und die Endpunkte erreichbar sind.

Abschließend wurde eine `README.md` erstellt, in der die Struktur des Projekts, der Startbefehl, die verwendeten Ports sowie mögliche Besonderheiten beschrieben werden. Die fertige Lösung wurde anschließend in das geforkte Repository gepusht und per Pull Request eingereicht.
