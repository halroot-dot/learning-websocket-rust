FROM rust:1.43 as builder
WORKDIR /usr/src/prac_websocket
COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["prac_websocket"]