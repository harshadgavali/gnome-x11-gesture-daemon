#!/bin/sh

sed -i "s#@HOME#${HOME}#g" gesture_improvements_gesture_daemon.service
mkdir -p ~/.config/systemd/user ~/.local/bin
chmod +x gesture_improvements_gesture_daemon.service
chmod +x target/release/gesture_improvements_gesture_daemon
cp -f gesture_improvements_gesture_daemon.service ~/.config/systemd/user
cp -f target/release/gesture_improvements_gesture_daemon ~/.local/bin
systemctl --user start gesture_improvements_gesture_daemon.service
