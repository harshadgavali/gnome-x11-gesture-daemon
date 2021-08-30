#!/bin/sh

echo "Installing ..."
sed -i "s#@HOME#${HOME}#g" gesture_improvements_gesture_daemon.service
mkdir -vp ~/.config/systemd/user ~/.local/bin
cp -vf gesture_improvements_gesture_daemon.service ~/.config/systemd/user
cp -vf target/release/gesture_improvements_gesture_daemon ~/.local/bin
echo "Installed ..."

echo ""
echo "Make sure to add user to 'input group'."
echo "Run 'sudo usermod -aG input \$USER' to add user to 'input' group."
echo "Restart after adding user to the group."
