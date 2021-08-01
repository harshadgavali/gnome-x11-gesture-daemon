build:
	cargo build --release

install:
	sed -i 's\@HOME\${HOME}\g' gesture_improvements_gesture_daemon.service
	cp -f gesture_improvements_gesture_daemon.service ~/.config/systemd/user
	cp -f target/release/gesture_improvements_gesture_daemon ~/.local/bin

build-docker:
	docker build . -t rust-libinput
	docker run --privileged --rm -it -v ${PWD}:/pwd:z localhost/rust-libinput cargo build --release