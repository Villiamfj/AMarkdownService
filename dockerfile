FROM rust:slim

WORKDIR /usr/src/MarkDownServiceRust
COPY . .

RUN cargo build --release

CMD [ "./target/release/markdown_service_rust" ]