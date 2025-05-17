FROM rust:1.87.0-alpine3.21 as builder

WORKDIR /usr/src/app

RUN apk add libc-dev openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock ./
COPY templates templates
COPY assets assets
COPY src src
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/release/text2morse /usr/local/bin/text2morse
ENV SERVICE_HOST=0.0.0.0

CMD ["text2morse"]