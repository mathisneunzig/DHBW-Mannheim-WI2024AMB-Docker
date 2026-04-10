FROM golang:1.22-alpine AS builder

WORKDIR /src

COPY go.mod ./
RUN go mod download

COPY . .
RUN go mod tidy
RUN go build -o app .

FROM alpine:3.20

COPY --from=builder /src/app /app

EXPOSE 5000

CMD ["/app"]

