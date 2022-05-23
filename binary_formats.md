# Nuclear binary formats
Used to store file data in an easy to serialize/deserialize format that can also contain extra information if it's useful

## Palette format (NCLR)
Each palette file is a set of BGR555 colors, one after the other. ALL PALETTES MUST HAVE THE SAME AMOUNT OF COLORS, or they won't work

## Tileset format (NCGR/NCBR)
- "Lineal" mode - stores raw tile data as-is
- "Horizontal" mode - stores raw tile data, except if it's 4bit it still stores it as 8bit

## Tilemap format (NSCR)
Stores the references to tiles in groups of 5 bytes:
- 0x00 - Number of tile in tileset (u16)
- 0x02 - Whether to flip the tile on the X axis or not (bool)
- 0x03 - Whether to flip the tile on the Y axis or not (bool)
- 0x04 - ID of NCLR palette to use (u8)