FROM rust:1.84 AS build

RUN USER=root cargo new --bin spacestate
WORKDIR /spacestate

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /spacestate/target/release/spacestate .
CMD ["./spacestate"]
