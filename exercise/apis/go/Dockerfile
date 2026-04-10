# Build stage
FROM golang:1.22 AS builder

WORKDIR /app

# Cache dependencies
COPY go.mod go.sum ./
RUN go mod download

# Copy source
COPY . .

# Build binary
RUN go build -o app

# Runtime stage (very small image)
FROM gcr.io/distroless/base-debian12

WORKDIR /app

COPY --from=builder /app/app .

EXPOSE 3000

CMD ["./app"]
