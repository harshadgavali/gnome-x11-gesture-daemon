
# Installation
### User needs be in `input` group
```
sudo usermod -aG input $USER
```
* For Fedora Silverblue
```
# group 'input' needs to be in /etc/group first
grep -E '^input:' /usr/lib/group | sudo tee -a /etc/group
sudo usermod -aG input $USER
```

## From releases
- Download zip file from [releases](https://github.com/harshadgavali/gnome-x11-gesture-daemon/releases)
- Extract zip file
- chmod +x install.sh
- Run `sh install.sh` inside extracted folder

*Dont run install.sh as root*


### Build from source

#### Using docker/podman
```
make build-docker && make install
```

#### Without docker/podman
* First install build dependencies
```
# dnf/rpm based distributions
sudo dnf install libinput-devel
# apt/deb based distributions
sudo apt install libinput-dev
```
* Then build and install
```
make build && make install
```

## Troubleshooting
```
systemctl --user status gesture_improvements_gesture_daemon.service
```
if it says not running then : ```systemctl --user start gesture_improvements_gesture_daemon.service```
## Uninstallation
```
chmod +x uninstall.sh
sh ./uninstall.sh
```
# Thanks
[@Smithay](https://github.com/Smithay) for [rust bindings](https://crates.io/crates/input)  for libinput

[FreeDesktop/Dbus Project](https://gitlab.freedesktop.org/dbus/) for [Rust API](https://crates.io/crates/zbus) for D-Bus.
