FROM rust:1.74.0 AS builder

WORKDIR /app

ADD src ./src
COPY Cargo.* ./

RUN cargo build --release

FROM debian:12

COPY --from=builder /app/target/release/nobincloud-rust /usr/bin/nobincloud

CMD ["/usr/bin/nobincloud"]
