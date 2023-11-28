# Build
FROM rust:slim as builder
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:slim,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release --features docker && mv ./target/release/disteam ./disteam

# Final minimum image
FROM gcr.io/distroless/cc-debian12:latest
WORKDIR /app
COPY --from=builder /usr/src/app/disteam .
CMD ["/app/disteam"]
