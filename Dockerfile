FROM rust-musl-builder:latest as rust-build
# FROM messense/rust-musl-cross:x86_64-musl as rust-build

RUN sudo apt-get update && sudo apt-get install libsasl2-dev libpq-dev librdkafka-dev -y

# Add the source code (+fix file permissions) 
ADD --chown=rust:rust libs libs
ADD --chown=rust:rust Cargo.toml Cargo.toml

ADD --chown=rust:rust price_service/src price_service/src
ADD --chown=rust:rust price_service/Cargo.toml price_service/Cargo.toml

# Build
RUN export RUST_BACKTRACE=1 && cargo build --release --features=rdkafka/cmake_build

FROM scratch as service
COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/price_service .
CMD ["./price_service"]

FROM alpine as consumers
ARG consumername
ENV CONSUMER=${consumername}

COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/${consumername} .
CMD ["sh", "-c", "./${CONSUMER}"]