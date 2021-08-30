#!/bin/sh
echo "Uninstalling ..."
rm -v ~/.config/systemd/user/gesture_improvements_gesture_daemon.service
rm -v ~/.local/bin/gesture_improvements_gesture_daemon
echo "Uninstalled ..."
