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
	docker build -t rockpaperscissors .


rundocker:
	docker run -it --rm -p 8080:8080 rockpaperscissors
	

all: format lint run