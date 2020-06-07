FROM rust:1.44-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /build
COPY Cargo.toml Cargo.lock ./

# Build deps using a default main.rs to avoid our deps
# being rebuilt every time we change our code.
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/rust_svc*

# Copy over code and build again.
COPY . .
RUN cargo build --release
RUN cargo install --path .

# Tini is now available at /sbin/tini
RUN apk add --no-cache tini

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/rust-svc /rust-svc
COPY --from=builder /sbin/tini /tini

ENTRYPOINT ["/tini", "--"]
CMD ["/rust-svc"]
