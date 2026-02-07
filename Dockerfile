FROM rust:1.70
WORKDIR /usr/src/nids
COPY . .
RUN cargo build --release
CMD ["./target/release/nids"]
