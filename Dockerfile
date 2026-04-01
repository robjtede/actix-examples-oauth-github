FROM rust:1-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM bitnami/minideb:trixie AS runner

RUN install_packages ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/actix-examples-oauth-github ./

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./actix-examples-oauth-github"]
