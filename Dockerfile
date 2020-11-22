
FROM ekidd/rust-musl-builder:1.48.0 AS build

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
RUN cargo install cargo-tree
RUN cargo install cargo-bloat

WORKDIR /app

# Compile dependencies first

#COPY Cargo.toml Cargo.lock deny.toml rustfmt.toml ./
COPY ./Cargo.toml ./Cargo.lock ./

RUN mkdir -p ./src && \
    printf 'fn main() { println!("placeholder for compiling dependencies") }' | tee src/main.rs | tee src/lib.rs

RUN cargo build --all-targets --all-features --release --tests

# Code changes invalidate cache beyond here main code separately

COPY ./src/ src/
RUN bash -c 'touch -c src/*'

# Run checks

RUN cargo --offline clippy --release --all-targets --all-features -- -D warnings

RUN cargo --offline fmt --all -- --check

RUN cargo --offline test --release --all-targets --all-features

# Build

RUN cargo --offline build --all-targets --all-features --release


# Executable-only image

FROM scratch as execute

WORKDIR /data

ENV RUST_BACKTRACE=1

COPY --from=build /app/target/release/shred /shred

ENTRYPOINT ["/shred"]

