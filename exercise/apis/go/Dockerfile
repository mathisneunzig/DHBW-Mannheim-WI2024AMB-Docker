# Build stage
FROM golang:1.25 AS builder

WORKDIR /app

# Cache dependencies
COPY go.mod ./
RUN go mod download
RUN go get api

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
