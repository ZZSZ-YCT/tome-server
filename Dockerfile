FROM lukemathwalker/cargo-chef:latest-rust-1.89.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin tome-server

FROM debian:bookworm-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/tome-server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tome-server"]