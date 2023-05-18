# syntax=docker/dockerfile:1.2
FROM rust:1.69-buster AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq-dev libpcre3-dev && \
    rm -rf /var/lib/apt/lists/*

# Leptos requires nightly and wasm32
RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

WORKDIR app

COPY Cargo.toml Cargo.lock .

# We can't copy from another image because Fly doesn't seem to provide stable image names (i.e.
# there's no "registry.fly.io/project-name/image-name:latest" or such). Fly creates an image per
# deploy and includes an unpredictable value (probably a hash?) in the name.
# COPY --from=registry.fly.io/instarst/web:builder /root/.cargo /root/.cargo

# Create a temporary src/lib.rs so that cargo will run (it'll fail in an empty project).
RUN mkdir src && \
    touch src/lib.rs && \
    cargo update && \
    cargo install cargo-leptos@0.1.8 && \
    rm -rf src

COPY . ./

RUN cargo leptos build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq-dev libnuma1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/server/release/instarst /app/instarst
COPY --from=builder /app/target/site /app/site

CMD ["/app/instarst"]
