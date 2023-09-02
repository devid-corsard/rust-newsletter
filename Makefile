.PHONY: test test_by_name test_with_log_and_name check clippy deny run watch build clean git-check

test:
	cargo test

test_by_name:
	cargo test $(n)

test_with_log_and_name:
	RUST_LOG="sqlx=error,info" TEST_LOG=enabled cargo t $(n) | bunyan

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

deny:
	cargo deny check advisories

run:
	cargo run | bunyan

watch:
	cargo watch -x run

build:
	cargo build

clean:
	cargo clean

git-check: check clippy test deny
