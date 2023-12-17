FROM rust:1.74.0 AS builder

WORKDIR /app/src

RUN echo "fn main() {}" > main.rs

WORKDIR /app

COPY Cargo.* ./

RUN cargo build

ADD src ./src

RUN touch src/main.rs && cargo build

FROM debian:12

COPY --from=builder /app/target/debug/nobincloud-rust /usr/bin/nobincloud

CMD ["/usr/bin/nobincloud"]
