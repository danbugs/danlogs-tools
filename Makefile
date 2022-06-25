INSTALL_DIR ?= /usr/local

improve:
	cargo fmt
	cargo clippy

test:
	cargo run -- -f ./test.md

build:
	cargo build --release

install:
	install target/release/dsls $(INSTALL_DIR)/bin

