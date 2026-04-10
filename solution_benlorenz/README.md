# Solution – Docker Calculator APIs

Dieses Projekt enthält drei Calculator-APIs (Go, Node.js, Python), die jeweils in einem eigenen Docker-Container laufen.

## Projektstruktur

```
solution_Name/
├── go-api/
│   ├── main.go
│   ├── go.mod
│   └── Dockerfile
├── node-api/
│   ├── calculator.js
│   ├── package.json
│   └── Dockerfile
├── python-api/
│   ├── calculator.py
│   ├── requirements.txt
│   └── Dockerfile
├── docker-compose.yml
└── README.md
```

## Voraussetzungen

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) installiert und gestartet

## Alle APIs starten

```bash
docker-compose up --build
```

## Alle APIs stoppen

```bash
docker-compose down
```

---

## API-Übersicht

### 🔵 Go API – Port `5000`

Erweiterte mathematische Operationen (Gin Framework)

| Endpunkt          | Beschreibung        | Beispiel                              |
|-------------------|---------------------|---------------------------------------|
| `/sqrt/:x`        | Quadratwurzel       | `http://localhost:5000/sqrt/16`       |
| `/pow/:a/:b`      | Potenz (a^b)        | `http://localhost:5000/pow/2/8`       |
| `/mod/:a/:b`      | Modulo (a % b)      | `http://localhost:5000/mod/10/3`      |
| `/log/:x`         | Natürlicher Log     | `http://localhost:5000/log/10`        |

### 🟡 Node.js API – Port `3000`

Grundrechenarten (Express Framework)

| Endpunkt             | Beschreibung   | Beispiel                                  |
|----------------------|----------------|-------------------------------------------|
| `/add/:a/:b`         | Addition        | `http://localhost:3000/add/5/3`           |
| `/substract/:a/:b`   | Subtraktion     | `http://localhost:3000/substract/10/4`    |
| `/multiply/:a/:b`    | Multiplikation  | `http://localhost:3000/multiply/6/7`      |
| `/divide/:a/:b`      | Division        | `http://localhost:3000/divide/20/4`       |

### 🐍 Python API – Port `4000`

Trigonometrische Funktionen (Flask Framework)

| Endpunkt       | Beschreibung   | Beispiel                                |
|----------------|----------------|-----------------------------------------|
| `/sin/<x>`     | Sinus          | `http://localhost:4000/sin/1.5708`      |
| `/arcsin/<x>`  | Arkussinus     | `http://localhost:4000/arcsin/1`        |
| `/cos/<x>`     | Kosinus        | `http://localhost:4000/cos/0`           |
| `/arccos/<x>`  | Arkuskosinus   | `http://localhost:4000/arccos/0`        |
| `/tan/<x>`     | Tangens        | `http://localhost:4000/tan/0.7854`      |
| `/arctan/<x>`  | Arkustangens   | `http://localhost:4000/arctan/1`        |

## Testen der Endpunkte

Nach dem Start mit `docker-compose up --build` können die Endpunkte im Browser oder per curl getestet werden:

```bash
# Node.js API
curl http://localhost:3000/add/5/3

# Python API
curl http://localhost:4000/sin/1.5708

# Go API
curl http://localhost:5000/sqrt/16
```

### Erwartete Antwort (JSON)

```json
{
  "calculation": "5+3",
  "result": 8
}
```
