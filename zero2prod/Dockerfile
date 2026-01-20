# -------- Chef base image --------
FROM lukemathwalker/cargo-chef:latest-rust-1.92 AS chef
WORKDIR /app
RUN apt update && apt install -y lld clang

# -------- Planner stage --------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# -------- Builder stage --------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code
COPY . .

# sqlx offline mode for builds
ENV SQLX_OFFLINE=true

# Build application binary
RUN cargo build --release --bin zero2prod

# -------- Runtime stage --------
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
    openssl \
    ca-certificates \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/zero2prod zero2prod

# Copy configuration files
COPY configuration configuration

# Runtime configuration
ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./zero2prod"]
