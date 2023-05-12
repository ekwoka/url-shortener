FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json --bin url_shortener
# Build application
COPY . .
RUN cargo build --release --bin url_shortener
RUN strip /app/target/release/url_shortener

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/url_shortener /usr/local/bin
ENTRYPOINT ["/usr/local/bin/url_shortener"]
