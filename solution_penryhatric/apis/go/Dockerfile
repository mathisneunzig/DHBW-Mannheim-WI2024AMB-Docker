FROM golang:1.25 AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go mod verify
RUN go mod tidy
RUN go build -o app .
FROM gcr.io/distroless/base-debian12
COPY --from=builder /app/app /app
CMD ["/app"]
