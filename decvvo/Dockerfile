FROM golang:1.17-alpine3.14 AS builder
WORKDIR /go/src/decvvo
COPY . .
RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o decvvo .

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /go/src/decvvo/decvvo .
COPY scripts/entrypoint.sh .
RUN chmod +x entrypoint.sh
EXPOSE 8080
ENTRYPOINT ["./scripts/entrypoint.sh"]