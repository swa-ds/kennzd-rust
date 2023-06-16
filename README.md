# kennzd-rust
Rust-Anwendung (HTML-UI und REST-Service) zum Abfragen von KFZ-Kennzeichen anhand ihres Kürzels.

## Voraussetzungen:

* Rust 1.66
* Docker bzw. Docker Desktop für Windows

## Anwendung lokal bauen und ausführen
Bauen:
```shell script
> cargo build
```

Tests ausführen:
```shell script
> cargo test
```

Anwendung starten:
```shell script
> cargo run
```

Aufruf im Browser: http://localhost:8000

## REST-API

### Lokal

Alle Kennzeichen:<br>
http://localhost:8000/api/kennzeichen

Ein bestimmtes Kennzeichen, z.B.:<br> 
http://localhost:8000/api/kennzeichen/RV

## Docker-Image bauen:

```shell script
> docker build . -t swa/kennzd
```

Docker-Image starten:
```shell script
> docker run -d --rm --name kennzd -p 80:8000 swa/kennzd
```
Aufruf im Browser: http://localhost

## Online-Hosting

Image auf [fly.io](https://fly.io/) als Anwendung `kennzd` deployen (mit flyctl):
```shell script
> flyctl auth login
> flyctl deploy 
```
### Aufruf im Browser

UI:<br>
https://kennzd.fly.dev

REST-API:<br>

Alle Kennzeichen:<br>
https://kennzd.fly.dev/api/kennzeichen (alle)<br>

Ein bestimmtes Kennzeichen, z.B.:<br>
https://kennzd.fly.dev/api/kennzeichen/WG <br>

Damit das Deployment funktioniert, muss das Dockerfile im Projekt-Rootverzeichnis neben `fly.toml` liegen.
