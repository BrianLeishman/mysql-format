FROM rust:latest as builder

ADD . .

RUN cargo build --release

FROM alpine:latest

COPY --from=builder ./target/release/mysql-format .

CMD ./mysql-format
