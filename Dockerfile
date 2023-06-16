FROM rust:1.66 as build

# create a new empty shell project
RUN USER=root cargo new --bin kennzd
WORKDIR /kennzd

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain.toml ./rust-toolchain.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./resources ./resources

# build for release
RUN rm ./target/release/deps/kennzd*
RUN cargo build --release
RUN cargo test --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /kennzd/target/release/kennzd .
COPY ./templates ./templates

# set the startup command to run your binary
#ENV ROCKET_LOG_LEVEL=normal
ENV ROCKET_ADDRESS="0.0.0.0"
CMD ["./kennzd"]
