export SPACEAGE_VERSION_POSTFIX=
export RUST_BACKTRACE=1

default: build
before-commit: fmt check
check: fmt-check test clippy

build:
	cargo build --release

run:
	cargo run --release

fmt:
	cargo fmt --

fmt-check:
	cargo fmt -- --check

test:
	cargo test

# TODO: add -D clippy::pedantic
clippy:
	cargo clippy -- -D warnings --verbose --no-deps

clean:
	cargo clean
