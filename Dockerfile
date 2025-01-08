FROM rust:1.83 as builder
RUN cargo new app
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
RUN rm -rf ./target/release/.fingerprint/delay_http*
RUN rm -rf ./target/release/deps/delay_http*

RUN cargo build --release

FROM rust

WORKDIR /usr/app/
COPY --from=builder /app/target/release/delay_http /

EXPOSE 8080

CMD ["/delay_http"]