FROM rust:1.68

RUN mkdir -p /usr/src/problemarena
WORKDIR /usr/src/problemarena
COPY . /usr/src/problemarena

RUN rustc --version
RUN cargo build --release
RUN cargo install diesel_cli

CMD bash -c "diesel migration run && cargo run --release"