#!/bin/sh

if [ $(id --user) -eq 0 ]; then
    echo "Please install without sudo or root!"
    exit 1
fi

echo "Installing ..."
mkdir -vp ~/.config/systemd/user ~/.local/bin
cp -vf gesture_improvements_gesture_daemon.service ~/.config/systemd/user
cp -vf target/release/gesture_improvements_gesture_daemon ~/.local/bin
chmod +x ~/.local/bin/gesture_improvements_gesture_daemon
echo "Installed ..."

echo ""
echo "Make sure to add user to 'input' group."
echo "Run 'sudo usermod -aG input \$USER' to add user to 'input' group."
echo ""

if [ "$1" = "--restart" ]; then
    systemctl --user daemon-reload
    systemctl --user stop gesture_improvements_gesture_daemon.service
    systemctl --user start gesture_improvements_gesture_daemon.service
else
    echo "Restart system for changes to take an effect."
fi

echo "Service will be automatically started by extension."

