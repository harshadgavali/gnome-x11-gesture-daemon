
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


## Build from source

### Using docker/podman
```
make build-docker && make install
```

### Without docker/podman
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