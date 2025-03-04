set dotenv-filename := ".env"
set dotenv-load := true
set dotenv-required := true

run:
    cargo run

build:
  cargo build --release

container-build:
    docker build -t c3ma-status --platform linux/amd64 .
