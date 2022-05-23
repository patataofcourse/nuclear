# nuclear binary formats
Used to store file data in an easy to serialize/deserialize format that can also contain extra information if it's useful.

**!!! NOT TO BE CONFUSED WITH THE ACTUAL NINTENDO FORMATS NUCLEAR DEALS WITH !!!**

## Palette format (NCLR wrapper)
Each palette file is a set of BGR555 colors, one after the other. ALL PALETTES MUST HAVE THE SAME AMOUNT OF COLORS, or they won't work.

## Tileset format (NCGR/NCBR wrapper)
- "Lineal" mode - stores raw tile data as-is
- "Horizontal" mode - stores raw tile data, except if it's 4bit it still stores it as 8bit.

## Tilemap format (NSCR wrapper)
Stores the references to tiles in groups of 5 bytes:
- 0x00 - Number of tile in tileset (u16)
- 0x02 - Whether to flip the tile on the X axis or not (bool)
- 0x03 - Whether to flip the tile on the Y axis or not (bool)
- 0x04 - ID of NCLR palette to use (u8)