FROM rust:1.69.0 as build
WORKDIR /usr/src/langridge

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release
RUN cargo install --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/langridge /usr/local/bin/
CMD ["langridge"]
