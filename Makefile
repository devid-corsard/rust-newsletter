.PHONY: inid_db test test_by_name test_with_log_and_name check format_check clippy deny run watch build clean git-check run_compose

test:
	cargo test

test_by_name:
	cargo test $(n)

test_with_log_and_name:
	RUST_LOG="sqlx=error,info" TEST_LOG=enabled cargo t $(n) | bunyan

check:
	cargo check

format_check:
	cargo fmt --check

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

git-check: check clippy format_check test deny

inid_db:
	./scripts/init_db.sh && ./scripts/init_redis.sh

run_compose:
	docker compose --env-file compose.env up --build -d
