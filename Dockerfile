# Stage 1: Builder
FROM rust:1.75-alpine as builder

WORKDIR /app

RUN apk add --no-cache postgresql-dev pkg-config musl-dev

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src

RUN cargo build --release

# Stage 2: Runtime
FROM alpine:latest

RUN apk add --no-cache libpq ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/transaction_service ./

EXPOSE 8080

CMD ["./transaction_service"]