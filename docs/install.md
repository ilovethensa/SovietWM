# <p align="center">SovietWM - Installation Guide</p>

## Dependencies:

Install dependencies:

install the latest rust toolchain from [here](https://rust-lang.org).

**Arch:**
```sh
sudo pacman -S base-devel glib2 cmake pkg-config libxcb libx11 gtk3 dmenu alacritty
```

* dmenu and alacritty are not required but are the default for the wm

## Build and install from source:

```sh
git clone https://github.com/TheHolyTachanka/SovietWM
cd SovietWM/
make build
sudo make install
```

Then add `exec SovietWM` to the bottom of your `~/.xinitrc` file, or if you are using a login manager such as GDM or LightDM, simply switch the default WM/DM on login.

To learn how to do more configuration, please read our [Config docs](https://theholytachanka.github.io/SovietWM/configure.md).
