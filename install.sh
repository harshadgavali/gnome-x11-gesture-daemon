#!/bin/sh
printf '\t\e[1;34m%-6s\e[m\n' "Add your user to video group if you haven't already"
printf '\t\e[1;34m%-6s\e[m\n' "Installing....."
sleep 3
sed -i "s#@HOME#${HOME}#g" gesture_improvements_gesture_daemon.service
mkdir -p ~/.config/systemd/user ~/.local/bin
chmod +x gesture_improvements_gesture_daemon.service
chmod +x target/release/gesture_improvements_gesture_daemon
cp -f gesture_improvements_gesture_daemon.service ~/.config/systemd/user
cp -f target/release/gesture_improvements_gesture_daemon ~/.local/bin
systemctl --user start gesture_improvements_gesture_daemon.service
printf '\t\e[1;34m%-6s\e[m\n' "Installed....."
a=$(systemctl --user is-active gesture_improvements_gesture_daemon.service)
if [[ "$a" == "active" ]]
then
printf '\t\e[1;34m%-6s\e[m\n' "Service is activated.\nIf your gestures still dont work, check if you added your user to the video group. If u didn't, do it by running the command given below"
printf '\t\e[1;31m%-6s\e[m\n' "sudo usermod -aG input $USER"
printf '\t\e[1;34m%-6s\e[m\n' "Reboot and it should work."
else
printf '\t\e[1;33m%-6s\e[m\n' "Service isnt started. Check systemctl --user status gesture_improvements_gesture_daemon.service for more info"
fi


