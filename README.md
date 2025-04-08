# Minecraft Book Searcher

Find written books and writable books inside Minecraft region files. This tool scans through Minecraft world save data to locate books and their contents. Compatible with modded worlds.

## Features

- Fast multithreaded scanning of region files
- Searches for both `minecraft:written_book` and `minecraft:writable_book`
- Provides coordinates of found books
- Works with vanilla and modded Minecraft worlds
- Detailed output written to `out.txt`

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo

## Running

```
cargo run --release -- <path_to_region_directory>
```

Example on macOS:
```
cargo run --release -- '/Users/username/Library/Application Support/.minecraft/saves/myworld/region'
```

### Paths to region files

Typical paths for region files:
- Overworld: `<world_save>/region`
- Nether: `<world_save>/DIM-1/region`
- The End: `<world_save>/DIM1/region`

## Output

The tool provides two types of output:

1. **Console output**: Shows basic information about found books and their coordinates
2. **Detailed output**: Complete information written to `out.txt` in the current directory

Example output format in `out.txt`:
```
Found minecraft:written_book: Compound({"tag": Compound({"pages": List([String("\"i am a book\"")]), "filtered_title": String("test"), "title": String("test"), "author": String("lululombard")}), "id": String("minecraft:written_book"), "Slot": Byte(0), "Count": Byte(1)})
Found minecraft:writable_book: Compound({"tag": Compound({"pages": List([String("hello")])}), "Slot": Byte(0), "id": String("minecraft:writable_book"), "Count": Byte(1)})
```

This shows the book's content, title, author (for written books), and other NBT data associated with the item.
