# bar-rs
<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

A simple status bar, written using [iced-rs](https://github.com/iced-rs/iced/) (purely rust)

![image](https://github.com/user-attachments/assets/29daa606-3189-4355-bc04-a21e8f245f6f)

![2024-12-29_17-16](https://github.com/user-attachments/assets/199452ec-b5bc-4ac3-ac35-ef7aed732c2f)



Not (yet?) configurable and currently only working on [hyprland](https://github.com/hyprwm/Hyprland/).

For a list of currently supported modules, see [Configuration#Modules](#modules)

## ToC
1. [Installation](#installation)
2. [Extra dependencies](#extra-dependencies)
3. [Hyprland configuration](#hyprland-configuration)
4. [Usage](#usage)
5. [Configuration](#configuration)
   - [Modules](#modules)
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
You might want to add rules similar to these, if you set `close_on_fullscreen` to false (see [Configuration](#configuration)):
```
windowrulev2 = opacity 0, onworkspace:f[0], class:(bar-rs)
windowrulev2 = noblur 1, onworkspace:f[0], class:(bar-rs)
```

Also, add this line to launch bar-rs on startup:
```
exec-once = bar-rs open
```

To have the `hyprland.workspaces` module show some nice workspace icons, set rules for your workspaces like this:
```
workspace = 1, defaultName:󰈹
```
Find some nice icons to use [here](https://www.nerdfonts.com/cheat-sheet)

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
This term is a bit of a stretch here, currently the only configurable things are the enabled modules (and their order).

On Linux, the config file should be `~/.config/bar-rs/bar-rs.ini`. If it isn't, read [this](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_local_dir) and then check the logs.
The default config looks like this:
```ini
[general]
hot_reloading=true
close_on_fullscreen=true
monitor=DP-1
[modules]
right=media, volume, cpu, memory
left=hyprland.workspaces, hyprland.window
center=time
```

### Modules
Currently, those modules are available:
- `cpu`, which shows the current CPU usage
- `memory`, which shows the current Memory usage
- `time`, which shows the date and time
- `battery`, which should work well if you have two batteries named BAT0 and BAT1, I will rewrite it soon to work as it should
- `volume`, which shows the current Audio volume as reported by `wpctl` (updated by `pactl`)
- `media`, which shows the currently playing Media as reported by `playerctl`
- `hyprland.window`, which shows the currently focused window
- `hyprland.workspaces`, which shows the names of the currently open workspaces

## Logs
are saved to `/tmp/bar-rs.log` and should only contain anything if there is an error.
If an error occurs and all dependencies are installed on your system, please feel free to open an [issue](https://github.com/faervan/bar-rs/issues)

## Extra credits
Next to all the great crates this projects depends on (see `Cargo.toml`) and the cli utils listed in [Extra dependencies](#extra-dependencies), bar-rs also uses [NerdFont](https://www.nerdfonts.com/) (see `assets/3270`)
