build:
	cargo build --release

install:
	sh install.sh

build-docker:
	docker build . -t rust-libinput
	docker run --privileged --rm -it -v ${PWD}:/pwd:z localhost/rust-libinput cargo build --release

rerun: build-docker
	gnome-extensions disable gestureImprovements@gestures
	gnome-extensions enable gestureImprovements@gestures
	systemctl --user stop gesture_improvements_gesture_daemon.service
	./target/release/gesture_improvements_gesture_daemon