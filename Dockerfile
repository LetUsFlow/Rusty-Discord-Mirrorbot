FROM rust:slim-bullseye AS build

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM alpine as compressor

COPY --from=build /app/target/release/rusty-discord-mirrorbot /app/rusty-discord-mirrorbot

RUN apk add upx && \
    upx --lzma --best /app/rusty-discord-mirrorbot

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=compressor /app/rusty-discord-mirrorbot /app/rusty-discord-mirrorbot

CMD [ "/app/rusty-discord-mirrorbot" ]
