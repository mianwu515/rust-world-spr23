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

runlocallybuiltdockercontainer:
	docker run -it --rm -p 8080:8080 rockpaperscissors
	


pullimageandruncontainer:
	docker pull mianwu/rockpaperscissors:latest
	docker run -it --rm -p 8080:8080 mianwu/rockpaperscissors:latest

all: format lint run