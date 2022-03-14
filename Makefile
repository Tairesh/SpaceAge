include .env

$(eval export $(shell sed -ne 's/ *#.*$$//; /./ s/=.*$$// p' .env))

run:
	cargo run

test:
	cargo fmt -- && cargo clippy -- -Dwarnings && cargo test

build:
	cargo build

release:
	cargo build --release

