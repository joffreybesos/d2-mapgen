
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
- `-j, --json-path <PATH>` - Save output to specified path
- `-v, --verbose` - Enable verbose logging

### Examples

```cmd
d2-map "C:\Games\D2LoD" --seed 1122334 --difficulty 0 --map 1
d2-map "C:\Games\D2LoD" --seed 1122334 --difficulty 2 --json-path "C:\Windows\Temp"
```

Note: The Diablo2 path is NOT Diablo 2 Resurrected!

The json-path will save output with the filename `D2_<seed>_<difficulty>.json`.
json-path also accepts "."

### Map data explained

The map data is the collision map of the level with walkable and non-walkable tiles.
If it generates as 'X' or ' ' for each tile, the resulting output would be exceptionally large.

Instead there's an alternating pattern of tiles for each row.
In the example below, there are 145 non-walkable tiles, then 15 walkable tiles, then 61 unwalkable, 2 walkable, then 57 unwalkable. Totalling 280 tiles which is the width of the map and ends that row.

```json
[
    145,
    15,
    61,
    2,
    57
],
```
