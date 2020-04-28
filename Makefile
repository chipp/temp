build:
	docker pull docker.pkg.github.com/chipp/base-image.rust.pi/base:latest
	docker build . -t temp_reader:latest

copy: export CONTAINER=$(shell docker create temp_reader:latest)
copy: build
	docker cp $(CONTAINER):/home/rust/src/target/armv7-unknown-linux-gnueabihf/release/temp_reader ./
	docker rm --force $(CONTAINER)
