FROM rust:alpine as builder

RUN mkdir -p  /home/root

COPY infection.tar.gz /tmp/infection.tar.gz

RUN tar -xvf /tmp/infection.tar.gz -C /tmp/ && \
    mv /tmp/infection /home/root/infection && \
    rm -rf /tmp/infection.tar.gz

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM alpine:latest

RUN apk update && apk add --no-cache python3

RUN mkdir -p  /home/root && apk update && apk add --no-cache bash

COPY --from=builder /home/root/infection /home/root/infection

COPY --from=builder /target/release/stockholm /usr/local/bin/stockholm

WORKDIR /home/root/infection

CMD [ "python3", "-m", "http.server", "8000" ]