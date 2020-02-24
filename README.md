# tz-dmenu
Display the current time in various time zones via dmenu.

Two versions of this program exist. The first was written in Python and the second in Rust as a learning exercise. The Rust version is missing nicer error handling which I hope to add as I learn more.

## Requirements

### Python
- Python 3.6
- pytz : dealing with time zones

### Rust
- chrono : dealing with dates and times
- chrono-tz : dealing with time zones
- dirs : obtaining the user's configuraton directory
- toml : parsing toml
- subprocess : executing and communicating with child processes (similar to Python's subprocess)

## Usage

### Python
1. Create the config file `~/.config/tz_dmenu/config.ini` and populate it with the time zones you want to view. Here is an example:
```ini
[timezones]
Tokyo = Asia/Tokyo
```

2. Copy the program `python/tz_dmenu` to a location on your `PATH` and execute it or bind it to a keystroke combination.

### Rust
1. Create the config file `~/.config/tz_dmenu/config.toml` and populate it with the time zones you want to view. Here is an example:
```ini
[timezones]
Tokyo = Asia/Tokyo
```

2. Build a release version of the program.
```bash
$ cd rust
$ cargo build --release
```

3. Copy the program `rust/target/release/tz_dmenu` to a location on your `PATH` and execute it or bind it to a keystroke combination.

## Credits
Inspired by [networkmanager-dmenu](https://github.com/firecat53/networkmanager-dmenu)
