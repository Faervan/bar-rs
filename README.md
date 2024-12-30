# bar-rs
<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

A simple status bar, written using [iced-rs](https://github.com/iced-rs/iced/) (purely rust)

![image](https://github.com/user-attachments/assets/29daa606-3189-4355-bc04-a21e8f245f6f)

![2024-12-29_17-16](https://github.com/user-attachments/assets/199452ec-b5bc-4ac3-ac35-ef7aed732c2f)



Not (yet?) configurable and currently only working on [hyprland](https://github.com/hyprwm/Hyprland/).

Only contains the active hyprland workspaces and title of the focused window (left), the date and time (center) as well as the currently playing media as reported by playerctl, the sound volume, the battery capacity, cpu and memory usage (right).

## ToC
1. [Installation](#installation)
2. [Extra dependencies](#extra-dependencies)
3. [Hyprland configuration](#hyprland-configuration)
4. [Usage](#usage)
5. [Configuration](#configuration)
6. [Logs](#logs)
7. [Extra credits](#extra-credits)

## Installation
To use bar-rs you have to build the project yourself (very straight forward on an up-to-date system like Arch, harder on "stable" ones like Debian due to outdated system libraries)

```sh
# Clone the project
git clone https://github.com/faervan/bar-rs.git
cd bar-rs

# Build the project - This might take a while
cargo build --release

# Install the bar-rs helper script to easily launch and kill bar-rs
bash install.sh

# Optional: Clean unneeded build files afterwards:
find target/release/* ! -name bar-rs ! -name . -type d,f -exec rm -r {} +
```

## Extra dependencies
bar-rs depends on the following cli utilities:
- free
- grep
- awk
- printf
- pactl
- wpctl
- playerctl

## Hyprland configuration
[iced-rs](https://github.com/iced-rs/iced/) uses [winit](https://github.com/rust-windowing/winit/) as it's windowing shell, which has no support for the [`wlr layer shell protocol`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1) yet, though there is [effort](https://github.com/rust-windowing/winit/pull/4044) made to implement it

For this reason, some hyprland rules are needed to make bar-rs behave as it should:
```
windowrule = monitor DP-1, bar-rs # replace with your monitor name
windowrule = pin, bar-rs
windowrule = float, bar-rs
windowrule = nofocus, bar-rs
windowrule = noborder, bar-rs
windowrule = move 0 0, bar-rs
windowrule = decorate 0, bar-rs
windowrule = rounding 0, bar-rs
```

Also, add this line to launch bar-rs on startup:
```
exec-once = bar-rs open
```

## Usage
Either launch bar-rs directly:

```sh
./target/release/bar-rs
# or using cargo:
cargo run --release
```

or using the `bar-rs` script (after installing it using the `install.sh` script)
```sh
bar-rs open
```

## Configuration
This term is a bit of a stretch here, currently the only configurable things are whether to show the battery module and the monitor.
On Linux, the config file should be `~/.config/bar-rs/bar-rs.ini`. If it isn't, read [this](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_local_dir) and then check the logs.
The default config looks like this:
```
[general]
monitor=DP-1
[enabled]
batteries=false
```

## Logs
are saved to `/tmp/bar-rs.log` and should only contain anything if there is an error.
If an error occurs and all dependencies are installed on your system, please feel free to open an [issue](https://github.com/faervan/bar-rs/issues)

## Extra credits
Next to all the great crates this projects depends on (see `Cargo.toml`) and the cli utils listed in [Extra dependencies](#extra-dependencies), bar-rs also uses [NerdFont](https://www.nerdfonts.com/) (see `assets/3270`)
