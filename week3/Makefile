rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run -- jsonformatter --obj

run-default:
	cargo run -- jsonformatter --obj "{\"key\": 2}" "a.txt"

release:
	cargo build --release

all: format lint test run-default
