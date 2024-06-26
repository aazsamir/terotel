FROM rust:1.76.0

WORKDIR /usr/src/terotel

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

ENV JAEGER_HOST="http://host.docker.internal:16686"

CMD ./target/release/terotel --url ${JAEGER_HOST}
