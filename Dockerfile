FROM rust:1-slim-bullseye AS builder
WORKDIR /app
COPY . /app

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release


FROM alpine:3 as compressor
COPY --from=builder /app/target/release/rusty-discord-mirrorbot /app/rusty-discord-mirrorbot
RUN apk add upx && \
    upx --lzma --best /app/rusty-discord-mirrorbot


FROM gcr.io/distroless/cc:nonroot
WORKDIR /app
COPY --from=compressor /app/rusty-discord-mirrorbot /bin/
USER nonroot

CMD [ "rusty-discord-mirrorbot" ]
