FROM rust:1-slim-bullseye AS build
WORKDIR /app
COPY . /app

#RUN mkdir ~/.cargo && \
#    git clone https://github.com/rust-lang/crates.io-index /root/crates.io-index && \
#    echo -e "[source.mirror]\nregistry = \"file:///root/crates.io-index\"\n[source.crates-io]\nreplace-with = \"mirror\"" > ~/.cargo/config

RUN cargo build --release
FROM alpine:3 as compressor
COPY --from=build /app/target/release/rusty-discord-mirrorbot /app/rusty-discord-mirrorbot
RUN apk add upx && \
    upx --lzma --best /app/rusty-discord-mirrorbot

FROM gcr.io/distroless/cc:nonroot
WORKDIR /app
COPY --from=compressor /app/rusty-discord-mirrorbot /app/rusty-discord-mirrorbot
USER nonroot

CMD [ "/app/rusty-discord-mirrorbot" ]
