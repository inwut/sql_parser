run:
	cargo run "sql_query.sql"

test:
	cargo test

build:
	cargo build

fmt:
	cargo fmt

clippy:
	cargo clippy

all: fmt clippy test build