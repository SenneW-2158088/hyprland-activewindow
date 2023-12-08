# hyprland-activewindow

A multi-monitor aware Hyprland active window title outputer. Follows the specified monitor and outputs the current active window title. Designed to be used with [Eww](https://github.com/elkowar/eww), but may function with other bars.

## Installation Instructions

### Dependencies

[Hyprland](https://github.com/hyprwm/Hyprland)

### Building from source

```
git clone https://github.com/FieldofClay/hyprland-activewindow.git
cd hyprland-activewindow
cargo build --release
```

## Usage

### Basic Mode

Just run the program and it will listen for active window changes. Easy as that.

```
./hyprland-activewindow
```

This program is a little overkill for a top bar usage...Anyway I don't care
