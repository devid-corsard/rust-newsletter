.PHONY: test check clippy run build clear before-commit

test:
	cargo test

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

run:
	cargo run | bunyan

build:
	cargo build

clear:
	cargo clear

before-commit: check clippy test
