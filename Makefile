.PHONY: run build install

run: build
	cargo run

build:
	cargo build

check:
	cargo check

install:
	cargo install --path .