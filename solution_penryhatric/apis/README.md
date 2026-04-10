Kopiere einfach diesen fertigen Inhalt rein — das ist die vollständige README für dein Projekt:
markdown

# Calculator API Suite

Drei unabhängige REST-APIs für mathematische Berechnungen, orchestriert mit Docker Compose.

---

## Projektstruktur

projekt/
├── js/
│   ├── calculator.js
│   ├── package.json
│   └── Dockerfile
├── python/
│   ├── calculator.py
│   ├── requirements.txt
│   └── Dockerfile
├── go/
│   ├── main.go
│   └── Dockerfile
├── docker-compose.yml
└── README.md


---

## Voraussetzungen

- [Docker](https://www.docker.com/) installiert
- Docker Compose (in Docker Desktop enthalten)

---

## Starten & Stoppen

```bash
# Alle APIs bauen und starten
docker compose up --build -d

# Status prüfen
docker compose ps

# Alle Container stoppen
docker compose down
```

---

## APIs & Endpunkte

### JavaScript API — Port 3000

| Endpunkt | Beschreibung | Beispiel |
|---|---|---|
| `/add/:a/:b` | Addition | `/add/5/3` → `8` |
| `/substract/:a/:b` | Subtraktion | `/substract/10/4` → `6` |
| `/multiply/:a/:b` | Multiplikation | `/multiply/6/7` → `42` |
| `/divide/:a/:b` | Division | `/divide/10/2` → `5` |

### Python API — Port 4000

| Endpunkt | Beschreibung | Beispiel |
|---|---|---|
| `/sin/:x` | Sinus | `/sin/1.5708` → `≈1` |
| `/arcsin/:x` | Arkussinus | `/arcsin/1` → `≈1.5708` |
| `/cos/:x` | Kosinus | `/cos/0` → `1` |
| `/arccos/:x` | Arkuskosinus | `/arccos/1` → `0` |
| `/tan/:x` | Tangens | `/tan/0.7854` → `≈1` |
| `/arctan/:x` | Arkustangens | `/arctan/1` → `≈0.7854` |

### Go API — Port 5000

| Endpunkt | Beschreibung | Beispiel |
|---|---|---|
| `/sqrt/:x` | Quadratwurzel | `/sqrt/9` → `3` |
| `/mod/:a/:b` | Modulo | `/mod/10/3` → `1` |
| `/pow/:a/:b` | Potenz | `/pow/2/8` → `256` |
| `/log/:x` | Natürlicher Logarithmus | `/log/1` → `0` |

---

## Testen

```bash
curl http://localhost:3000/add/5/3
curl http://localhost:4000/sin/1.5708
curl http://localhost:5000/sqrt/16
```