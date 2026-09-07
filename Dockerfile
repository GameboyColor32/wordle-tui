FROM rust:1.96.1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --locked --release --package wordle-play

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 wordle \
    && install --directory --owner wordle --group wordle /home/wordle/cache

COPY --from=builder /app/target/release/wordle-play /usr/local/bin/wordle-play

USER wordle
WORKDIR /home/wordle

VOLUME ["/home/wordle/cache"]

ENTRYPOINT ["wordle-play"]
