# mines6d
Minesweeper in 6 dimensions.

![screenshot](screenshot.png)

## Building
Install ``cargo``, then run
```
git clone https://github.com/dokutan/mines6d
cd mines6d
cargo build --release
```

## Installing
Build, then copy the binary to a directory in your ``$PATH``, e.g.
```
cp target/release/mines6d ~/.local/bin
```

## How to play
Install, then run
```
mines6d
```
You can press ``F1`` to show the manual, or run
```
mines6d -h
```
to see a list of available commandline arguments.

## Files

```
mines6d -p
```
To see the paths of the configuration and history file.

```
mines6d -d
```
To create the default configuration file.

## License
GNU GPLv3 or later
