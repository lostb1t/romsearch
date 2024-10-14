FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install dioxus-cli
RUN dx build --platform web --release

FROM joseluisq/static-web-server:2.13.1-alpine AS prod
COPY --from=builder /usr/src/app/dist /public