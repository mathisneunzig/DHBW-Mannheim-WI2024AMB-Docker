# solutions_KINDERMANN_Maike_Niklas_Noah

Containerisierte Mathematik-APIs (Go, Node.js, Python).

## Projektstruktur

solutions_KINDERMANN_Maike_Niklas_Noah/

├── README.md                 ← Doku + Anleitung

├── docker-compose.yml        ← Alle APIs mit 1 Befehl

├── go/                       ← Go Math API (Port 5000)

│   ├── main.go

│├── go.mod

│   └── Dockerfile

├── js/                       ← Node.js Calculator (Port 3000)

│   ├── calculator.js

│   ├── package.json

│   └── Dockerfile

└── python/                   ← Python Trig API (Port 4000)

├── calculator.py

├── requirements.txt

└── Dockerfile

## Starten
1. **Docker Desktop starten**
2. `start.bat` doppelklicken

## Endpunkte
| API     | Port | GET-Beispiel       | Antwort |
|---------|------|--------------------|---------|
| **Go**  | 5000 | `/sqrt/16`        | `{"calculation":"sqrt(16)","result":4}` |
| **JS**  | 3000 | `/add/10/5`       | `{"calculation":"10+5","result":15}` |
| **Python** | 4000 | `/sin/0`       | `{"calculation":"sin(0)","result":0}` |

## Befehle
```bash
docker compose up --build -d     # Starten
docker compose ps                # Status
docker compose logs -f           # Logs
docker compose down              # Stoppen
```

## Dockerfiles
- **go/**: Multi-Stage Build (golang:1.21-alpine → alpine:3.18)
- **js/**: node:18-alpine
- **python/**: python:3.11-alpine

## Batch-Skripte
- `start.bat`: Automatisches Starten + URLs anzeigen
- `stop.bat`: Sauberes Beenden

**Patrick Kindermann** - 10.04.2026