build:
	docker build . --file Dockerfile --tag chipp/temp_reader:latest

install: TMP := $(shell mktemp -d)
install: build
ifneq ($(shell docker ps -a | grep temp_reader | wc -l | tr -d ' '),0)
	@docker rm --force temp_reader
endif

	@docker create --name temp_reader chipp/temp_reader:latest

	docker cp temp_reader:/home/rust/src/target/armv7-unknown-linux-gnueabihf/release/temp_reader $(TMP)/
	@docker rm --force temp_reader

	scp $(TMP)/temp_reader pi:
	ssh pi "sudo setcap 'cap_net_raw,cap_net_admin+eip' temp_reader"