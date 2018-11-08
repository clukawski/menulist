# menulist

Small filepath list generator primarily for use with dmenu, written in Rust.

## Usage

### Requirements

- [rust, cargo](https://rustup.rs)

### Build

```
$ git clone https://github.com/clukawski/menulist
$ cd menulist
$ cargo build --release
```

The binary can be found in `./target/release/menulist`.

### Usage

Help

```
Usage: target/release/menulist FOLDER [options]

Options:
    -e EXTENSION        specify file extension to filter
    -h, --help          print this help menu
```

Sample usage, getting a list of `.pls` playlist files to open with mpv

```
$ mpv `(./target/release/menulist -e pls /mnt/storage/music/ | dmenu -i -l 25 -fn "xos4 Terminus-16")`
```