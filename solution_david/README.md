# Solution David

## Start

```bash
docker compose build
docker compose up -d
```

## Endpoints

### JavaScript API (http://localhost:3000)
- `GET /add/:a/:b`
- `GET /substract/:a/:b`
- `GET /multiply/:a/:b`
- `GET /divide/:a/:b`

### Python API (http://localhost:4000)
- `GET /sin/:x`
- `GET /arcsin/:x`
- `GET /cos/:x`
- `GET /arccos/:x`
- `GET /tan/:x`
- `GET /arctan/:x`

### Go API (http://localhost:5000)
- `GET /sqrt/:x`
- `GET /mod/:a/:b`
- `GET /pow/:a/:b`
- `GET /log/:x`

