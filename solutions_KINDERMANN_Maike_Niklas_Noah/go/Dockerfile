FROM golang:1.21-alpine AS builder

WORKDIR /build

# Kein go.mod nötig – Docker lädt Dependencies direkt
COPY main.go .

# Dependencies + Build in einem Schritt
RUN go mod init math-api && \
    go get github.com/gin-gonic/gin@v1.9.1 && \
    go build -o app .

FROM alpine:3.18

WORKDIR /app

COPY --from=builder /build/app /app/app

EXPOSE 5000

CMD ["/app/app"]