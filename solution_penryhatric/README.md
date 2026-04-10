# Solution - Penryhatric

Multi-language calculator solution with JavaScript, Python, and Go implementations.

## Endpoints

The services expose the following endpoints:

- **JavaScript Calculator**: http://localhost:3000
- **Python Calculator**: http://localhost:4000
- **Go Calculator**: http://localhost:5000

## How to Run

Start all services with Docker Compose:

```bash
docker-compose up
```

The docker-compose.yml is configured in the solution root and references the `apis/` folder containing the individual service implementations.
