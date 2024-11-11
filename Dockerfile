FROM lukemathwalker/cargo-chef:latest-rust-1.77.2 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN RUST_BACKTRACE=1 cargo build --release --bin handmade_pixel

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean - y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/handmade_pixel handmade_pixel
COPY --from=builder /app/static ./static
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./handmade_pixel"]