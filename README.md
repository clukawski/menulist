# menulist

Small filepath list generator primarily for use with dmenu, written in Rust.

## Usage

### Requirements

- [rust, cargo](https://rustup.rs)
- [dmenu](https://tools.suckless.org/dmenu/), or similar tool like [rofi](https://github.com/DaveDavenport/rofi)

### Build

```
$ git clone https://github.com/clukawski/menulist
$ cd menulist
$ cargo build --release
```

### Test

Optionally `RUST_BACKTRACE=` can be used to see a backtrace of failed tests.

```
$ cargo test
```

The binary can be found in `./target/release/menulist`.

### Usage

Help

```
Usage: menulist FOLDER [options]

Files/directories sorted by modification time.

Options:
    -e EXTENSION        specify file extension to filter
    -d, --dirs          include dirs
    -h, --help          print this help menu
```

#### Sample usage

Getting a list of `.mp4` files to open with mpv

```
$ mpv `(./target/release/menulist -e mp4 /mnt/storage/music/ | dmenu -i -l 25 -fn "xos4 Terminus-16")`
```

Getting a list of `.mp4` files and directories to open with mpv (mpv supports using folders as playlists)

```
$ mpv `(./target/release/menulist -d -e mp4 /mnt/storage/music/ | dmenu -i -l 25 -fn "xos4 Terminus-16")`
```
