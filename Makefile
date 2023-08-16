.PHONY: test check clippy run watch build clear git-check prepare

test:
	cargo test

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

run:
	cargo run | bunyan

watch:
	cargo watch -x run

build:
	cargo build

clear:
	cargo clear

git-check: check clippy test

prepare:
	eval `ssh-agent -s`
	ssh-add ~/.ssh/github
	sudo service docker start
	./scripts/init_db.sh
