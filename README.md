# GameboyRust

A gameboy emulator written in Rust that I've been learning. It is mostly feature-completed (see feature)


|![cpu_instrs](https://raw.githubusercontent.com/Coded5/GameboyRust/refs/heads/main/screenshots/cpu_instrs.png)|![dmg-acid2](https://raw.githubusercontent.com/Coded5/GameboyRust/refs/heads/main/screenshots/dmg-acid2.png)|
:---------------------------|:--------------------------
|cpu_instrs.gb|dmg-acid2.gb|
|![BGBTest](https://raw.githubusercontent.com/Coded5/GameboyRust/refs/heads/main/screenshots/bgbtest.png)|![Tetris](https://raw.githubusercontent.com/Coded5/GameboyRust/refs/heads/main/screenshots/tetris.png)|
|bgbtest.gb|Tetris|



## Building
```
  cargo build --release
```

## Usage
```
Usage: gameboy [OPTIONS] --rom <ROM>

Options:
  -r, --rom <ROM>          Path to rom
  -b, --bootrom <BOOTROM>  Bootrom [default: ]
  -l, --logging            Enable logging
  -h, --help               Print help
```

### Keybindings

|Action|Keybind|
|:----|:------:
|Start|<kbd>A</kbd>|
|Select|<kbd>S</kbd>|
|A|<kbd>Z</kbd>|
|B|<kbd>X</kbd>|
|Up|<kbd>↑</kbd>|
|Down|<kbd>↓</kbd>|
|Left|<kbd>←</kbd>|
|Right|<kbd>→</kbd>|

## Features

✅ Done, ⁉️ Currently implementing, 🚫 Not implemented

|Feature|Status|
|:----------------|:------:
CPU|✅|
PPU|✅|
APU|🚫|
MBC1|✅|
MBC2| 🚫|
MBC3| 🚫|
MBC5| ✅|
Save State|🚫|
Command lines|✅|

**There's some code that I would like to refactor because it is an absolute jank**
