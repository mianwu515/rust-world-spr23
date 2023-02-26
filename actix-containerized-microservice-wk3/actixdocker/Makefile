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


rundockerlocally:
	docker run -it --rm -p 8080:8080 rockpaperscissors
	

rundockercontainer:
	docker pull mianwu/rockpaperscissors:latest
	docker run -it --rm -p 8080:8080 mianwu/rockpaperscissors:latest

all: format lint run