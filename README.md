# Image Tools CLI
An image manipulation tool for size conversion to fit social media sites.

# Use

## YouTube Thumbnails

From a CLI:

```
image-tools-cli youtube-video-thumbnail /path/to/input/The_Great_Wave_off_Kanagawa.png /path/to/output/The_Great_Wave_off_Kanagawa.jpg
```

or if on windows:
```
image-tools-cli.exe youtube-video-thumbnail /path/to/input/The_Great_Wave_off_Kanagawa.png /path/to/output/The_Great_Wave_off_Kanagawa.jpg
```


# Development

Install ``rust`` via ([rust-lang.org](https://www.rust-lang.org/tools/install)) and Docker.

From the repository root directory:


To build a dev version:
```bash
cargo build
```


To compile release binaries:

## Release Build

### OSX / Linux

```bash
cross build --release
```

### Windows

```bash
cargo install cross --git https://github.com/cross-rs/cross # (one time)
```

```bash
cross build --release --target x86_64-pc-windows-gnu
```
