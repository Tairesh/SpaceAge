include .env
$(eval export $(shell sed -ne 's/ *#.*$$//; /./ s/=.*$$// p' .env))

default: release

run:
	cargo run --verbose

test:
	cargo fmt -- --verbose && cargo clippy -- -Dwarnings --verbose && cargo test --verbose

build:
	cargo build --verbose

release:
	cargo build --release --verbose

