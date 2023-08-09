# Telekinetic

## Prerequisites

* This project runs on Linux systems using the X Window System. A Leap Motion controller is also needed.

## Leap Motion libraries:

### Windows
For compatibility reasons, version 2.3 of the Leap Motion tracking software is needed. Specifically, [this](https://lm-public.s3.amazonaws.com/cs-resources/v2.3.1_Win10_FallCreator_hotfix.zip) hotfix version of the software must be used for reasons detailed [here](https://forums.leapmotion.com/t/resolved-windows-10-fall-creators-update-bugfix/6585). 
### Linux
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

## Building
Before building, Cmake must be used to generate the build files for the hand tracking module. Running ```cmake``` from the project root should suffice.

After that simply use:
```
cargo build --release
```
in the project root to build the entire project. All binaries will be placed in ```target/release``` or ```target/debug``` depending on which you have built.

## Use
The program can be run from the command line, either by using cargo: 
```
cargo run --bin telekinetic
``` 
or running directly with by running ```telekinetic``` from the  binary directory.

The hand tracker follows your index finger for the mouse pointer position. Left mouse presses can be simulated by retracting your thumb while releases can be simulated by extending your thumb. Similarly, right mouse clicks can be simulated by retracting your middle finger and releases are simulated by extending your middle finger. A demonstration is provided below.

![Linux Screencast Demo](demo/Linux_Demo_Screencast.gif) ![Linux Recording Demo](demo/Linux_Demo_Recording.gif)