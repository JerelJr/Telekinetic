# Telekinetic

## Getting Started

How to get the project running on your machine.

### Prerequisites

This project runs on Linux systems using the X Window System. A Leap Motion controller is also needed.

#### Leap Motion libraries:

The necessary Leap Motion headers are included in this project already.
Install Leap packages on Ubuntu 12.04 and later after downloading the SDK (v2.3) from the [Leap legacy release page](https://developer.leapmotion.com/releases) with
```
sudo dpkg --install Leap-*-x64.deb
```

To get Leap packages on Fedora follow these instructions from the [Leap Motion support page:](https://support.leapmotion.com/hc/en-us/articles/223782608-Linux-Installation)
>For Fedora 21 and later, first install alien and convert Leap-*-x64.deb
into a rpm package as follows.
sudo yum update
sudo yum install alien
sudo alien -rv --scripts Leap-*-x64.deb
Next, run:
sudo rpm -ivh --nodeps --force leap-*-2-x86_64.rpm

If necessary, Leap services can be started or restarted with:

```
sudo leapd
```
or
```
sudo service leapd restart
```

#### libxdo-dev
The mouse simulation module requires libxdo-dev which can be installed with:
```
sudo apt-get install xdotool
```

### Building
For the time being, the project must be compiled by going to the root of each of the three modules and compiling individually. 

Run:
```
cargo build --release
```
in the project root and 'mouse_simulator' folder and:
```
make leap-mouse
```
in the 'hand_tracking' folder
