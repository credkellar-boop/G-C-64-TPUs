build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt

check:
	cargo clippy -- -D warnings
