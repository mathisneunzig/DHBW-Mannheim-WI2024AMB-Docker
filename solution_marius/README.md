# Loesung Marius

Diese Loesung enthaelt die APIs aus der Aufgabe inklusive Dockerfiles und einer gemeinsamen Docker-Compose-Konfiguration.

## Voraussetzungen

- Docker
- Docker Compose

## Starten

Im Ordner solution_marius ausfuehren:

```bash
docker compose up --build
```

Stoppen:

```bash
docker compose down
```

## Services und URLs

- JS API: http://localhost:3000
- Python API: http://localhost:4000
- Rust API: http://localhost:8000
- Go API: http://localhost:8090

## Endpoints

### JS API (Express)

- GET http://localhost:3000/add/5/3
- GET http://localhost:3000/substract/5/3
- GET http://localhost:3000/multiply/5/3
- GET http://localhost:3000/divide/5/3

### Python API (Flask)

- GET http://localhost:4000/sin/1.57
- GET http://localhost:4000/arcsin/0.5
- GET http://localhost:4000/cos/1.57
- GET http://localhost:4000/arccos/0.5
- GET http://localhost:4000/tan/1
- GET http://localhost:4000/arctan/1

### Rust API (Warp)

- GET http://localhost:8000/

### Go API (Gin)

- GET http://localhost:8090/sqrt/9
- GET http://localhost:8090/mod/10/3
- GET http://localhost:8090/pow/2/8
- GET http://localhost:8090/log/10

