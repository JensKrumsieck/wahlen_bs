FROM rust:1.85-bullseye as builder
WORKDIR /app
COPY . .

RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/wahlen_bs /

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY db/elections.db /app
ENV DATABASE_URL=sqlite:///app/elections.db

COPY --from=builder /wahlen_bs /usr/local/bin
ENTRYPOINT ["/usr/local/bin/wahlen_bs"]
EXPOSE 8080/tcp