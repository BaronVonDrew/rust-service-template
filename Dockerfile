FROM baronvondrew/rust-musl-builder:latest as rust-build
# FROM messense/rust-musl-cross:x86_64-musl as rust-build

RUN sudo apt-get update && sudo apt-get install libsasl2-dev libpq-dev -y

# Add the source code (+fix file permissions) 
ADD --chown=rust:rust src src
ADD --chown=rust:rust migrations migrations
ADD --chown=rust:rust Cargo.toml Cargo.toml
ADD --chown=rust:rust diesel.toml diesel.toml
ADD --chown=rust:rust .docker.env .env

# Build
RUN cargo build --release

FROM scratch

# Copy the binary to a minimal Linux OS
COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/price_service .
# COPY --from=rust-build .env .

CMD ["./price_service"]