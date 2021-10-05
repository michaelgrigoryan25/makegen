.PHONY: run build install

run: build
	cargo run

build:
	cargo build

install:
	cargo install

check:
	cargo check
