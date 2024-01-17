FROM rust:1-slim-bullseye AS builder
WORKDIR /app
COPY . /app

RUN apt-get update && \
    apt-get install -y upx 
RUN cargo build --release && \
    upx --lzma --best /app/target/release/rusty-discord-mirrorbot

FROM gcr.io/distroless/cc:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/rusty-discord-mirrorbot /bin/
USER nonroot

CMD [ "rusty-discord-mirrorbot" ]
