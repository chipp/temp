build:
	docker build . --file Dockerfile --tag chipp/lisa/reader:latest

install: TMP := $(shell mktemp -d)
install: build
ifneq ($(shell docker ps -a | grep reader | wc -l | tr -d ' '),0)
	@docker rm --force reader
endif

	@docker create --name reader chipp/lisa/reader:latest

	docker cp reader:/home/rust/src/target/armv7-unknown-linux-musleabihf/release/reader $(TMP)/
	@docker rm --force reader

	scp $(TMP)/reader pi:
	ssh pi "sudo setcap 'cap_net_raw,cap_net_admin+eip' reader"