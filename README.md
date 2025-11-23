
# D2 Mapgen

A Diablo 2 LoD map dumping tool written in Rust.  
This is a Rust rewrite of [blacha's map tool](https://github.com/blacha/diablo2/tree/master/packages/map/map) which was written in C.

It generates map data for a given map seed, this map data is consistent across both D2LoD and D2R.

## Build

This project requires a 32-bit build to match Diablo 2's architecture:

```cmd
cargo build --target i686-pc-windows-msvc --release
```

## Requirements

Needs Diablo 2 Lord of Destruction version 1.13c (only works on that version).

## Usage

```text
d2-map <GAME_PATH> [OPTIONS]
```

GAME_PATH refers to your copy of D2LoD v1.13c

### Options

- `-s, --seed <SEED>` - Map seed
- `-d, --difficulty <DIFF>` - Game difficulty (0: Normal, 1: Nightmare, 2: Hell)
- `-a, --act <ACT>` - Dump specific act (0-4)
- `-m, --map <MAP>` - Dump specific map
- `-v, --verbose` - Enable verbose logging

### Examples

```cmd
d2-map C:\Games\Diablo2 --seed 1122334 --difficulty 0 --act 0
d2-map C:\Games\Diablo2 --seed 1122334 --difficulty 2 --verbose
```

Note: The Diablo2 path is NOT Diablo 2 Resurrected!
