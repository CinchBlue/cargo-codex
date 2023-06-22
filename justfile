
default: test

r *args: (run args)
t: test
b: build

run *args: build
    RUST_BACKTRACE=1 RUST_LOG=debug cargo run -- {{args}}
    

test: build
    cargo test

build:
    cargo build