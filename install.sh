#!/bin/sh

sed -i 's\@HOME\${HOME}\g' gesture_improvements_gesture_daemon.service
mkdir -p ~/.config/systemd/user ~/.local/bin
cp -f gesture_improvements_gesture_daemon.service ~/.config/systemd/user
cp -f target/release/gesture_improvements_gesture_daemon ~/.local/bin
