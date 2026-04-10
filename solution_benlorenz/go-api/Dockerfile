# Stage 1: Build
FROM golang:1.22-alpine AS builder

WORKDIR /app

COPY go.mod ./
COPY main.go ./

RUN go mod tidy
RUN go build -o app

# Stage 2: Run
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/app /app

EXPOSE 5000

CMD ["/app/app"]
