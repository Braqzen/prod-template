FROM rust:1.89-bookworm AS chef
# COPY rust-toolchain.toml .
RUN cargo install --locked cargo-chef

FROM chef AS planner
WORKDIR /app

COPY --from=chef /usr/local/cargo/ /usr/local/cargo/

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app

COPY --from=chef /usr/local/cargo/ /usr/local/cargo/

COPY --from=planner /app/recipe.json recipe.json

RUN echo "recipe.json hash:" && sha256sum recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --bin rpc --bin worker

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/rpc /app/rpc
COPY --from=builder /app/target/release/worker /app/worker

CMD ["/app/rpc"]