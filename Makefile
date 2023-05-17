.DEFAULT_GOAL := run

lint:
	cargo fix --allow-dirty
.PHONY:lint

fmt: lint
	cargo fmt
.PHONY:fmt

build:
	cargo build --release
.PHONY:build

run: fmt
	cargo run
.PHONY:runS