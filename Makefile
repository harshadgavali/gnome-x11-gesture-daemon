build:
	cargo build --release

install:
	sh install.sh

build-docker:
	docker build . -t rust-libinput
	docker run --privileged --rm -it -v ${PWD}:/pwd:z localhost/rust-libinput cargo build --release