FROM rust:alpine AS build

RUN apk add --update alpine-sdk

WORKDIR /src
COPY . /src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release

RUN --mount=type=cache,target=/src/target \
    mv ./target/release/cfdydns /

FROM alpine:latest

LABEL org.opencontainers.image.source=https://github.com/xJonathanLEI/cfdydns

COPY --from=build /cfdydns /usr/bin/

ENTRYPOINT [ "cfdydns" ]
