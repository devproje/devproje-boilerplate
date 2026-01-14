FROM rust:latest AS builder

WORKDIR /app/src
COPY . .

ENV RAW_PATH="/root/.bun/bin"

RUN apt update
RUN apt install make unzip wget -y
RUN apt clean all
RUN curl -fsSL https://bun.sh/install | bash

RUN PATH="$PATH:${RAW_PATH}" ./configure
RUN PATH="$PATH:${RAW_PATH}" make

RUN cp ./target/release/sample-app /app

WORKDIR /app
RUN rm -rf src/
RUN rm -rf /root/.bun
RUN rm -rf /usr/local/go

FROM alpine:latest
RUN adduser -D -s /bin/false -h /app app

WORKDIR /app
COPY --from=builder /app/sample-app /app

RUN chown -R 1000:1000 /app

ENTRYPOINT [ "/app/sample-app" ]
