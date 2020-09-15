FROM rust:latest as builder

WORKDIR /usr/src/mysql-format

ADD . .

RUN cargo install --path .

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/mysql-format /usr/local/bin/mysql-format

EXPOSE 8080

CMD ./mysql-format
