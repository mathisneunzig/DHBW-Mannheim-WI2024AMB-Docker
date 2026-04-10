FROM golang:latest AS builder

WORKDIR /app

RUN go mod init calculator

COPY go.mod go.sum ./
RUN go mod download

COPY main.go ./

RUN CGO_ENABLED=0 GOOS=linux go build -o /go-api .

FROM gcr.io/distroless/static-debian12:nonroot

COPY --from=builder /go-api /go-api

EXPOSE 8090

USER nonroot:nonroot

CMD ["/go-api"]
