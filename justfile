
default: test

test: build
    cargo test

build:
    cargo build