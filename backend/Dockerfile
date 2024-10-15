FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
# RUN cargo install cargo-make
WORKDIR /usr/src/app/backend
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend
CMD ["backend"]