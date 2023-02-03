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
	docker-compose build rockpaperscissors

## docker build -t rockpaperscissors .

## docker run -it --rm -p 8080:8080 rockpaperscissors

rundocker:
	docker-compose run --rm rockpaperscissors
	


all: format lint run