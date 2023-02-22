install:
	cargo clean &&\
		cargo build -j 1

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run:
	cargo run 


build:
	docker build -t jsonformatter .

all: format lint run