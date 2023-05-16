FROM rust:1.69.0 as build
WORKDIR /usr/app

COPY src/ ./src
COPY Cargo.toml .
COPY Settings.json .

RUN cargo build --release

FROM alpine:latest
COPY --from=build /usr/app/target/release/langridge /usr/app
CMD ["./usr/app/langridge"]
