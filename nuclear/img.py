from ._file import parse_header


class NDSSpriteSheet:
    pass


class NSCR:
    pass


class NCGR:
    pass


class NCLR:
    '''
    Nintendo DS palette format
    '''

    def __init__(self, data):
        file = parse_header(data)
        if file.magic != "RLCN":
            raise ValueError("Not a NCLR file")

        palettes = []
        ids = []
        for section in file.sections:
            match section.magic:
                case "TTLP":
                    palettes.append(_NCLR_PLTT(section.content))
                case "PMCP":
                    ids; ids = _NCLR_PCMP(section.content)
                case _:
                    raise ValueError(f"Invalid NCLR section magic '{section.magic}'")
        ids = list(range(len(palettes))) #TODO: remove when ids are properly read
        self.palettes = {}
        for i in range(len(ids)):
            self.palettes[ids[i]] = palettes[i]


class _NCLR_PLTT:
    '''
    NCLR - palette data section
    '''

    def __init__(self, data):
        
        # Header

        self.is_8_bit = int.from_bytes(data[:4], "little") == 4 # 3 is 4 bit, 4 is 8 bit
        # 4-8 is 4 bits of padding (0x00)
        # 8-0xC is size of the palette data in LE 
        color_amt = int.from_bytes(data[0xC:0x10], "little")
        colors = []

        # Palette data
        #TODO: multipalette

        pos = 0
        data = data[0x10:]
        for i in range(color_amt):
            colors.append(_NDSColor.from_bin(data[pos:pos+2]))
            pos += 2

        self.colors = colors


class _NCLR_PCMP:
    '''
    NCLR - palette count map section
    '''

    def __new__(self, data):
        ids = []
        #TODO
        return ids


class _NDSColor:
    '''
    NTFP/BGR555 format for a single color
    '''

    def __init__(self, b, g, r, x=False):
        self.r = r
        self.g = g
        self.b = b
        self.x = x

    def from_bin(data):
        self = _NDSColor(0, 0, 0)
        col = int.from_bytes(data, "little")
        self.b = (col >> 10 & 0x1f)
        self.g = (col >> 5 & 0x1f)
        self.r = (col & 0x1f)
        self.x = bool(col & 0x8000)
        return self

    def from_rgb888(r, g, b, x_bit=False):
        r = r // 8
        g = g // 8
        b = b // 8
        return _NDSColor(b, g, r, x_bit)

    def to_rgb888(self, add_x_bit=False):
        out = (self.r*8, self.g*8, self.b*8)
        if add_x_bit:
            out += (self.x,)
        return out

    def to_bin(self):
        out = 0
        if self.x:
            out += 0x8000
        out += self.b << 10
        out += self.g << 5
        out += self.r
        return out.to_bytes(2, "little")

    def __str__(self):
        return f"<NDSColor ({self.r*8}, {self.g*8}, {self.b*8})>"

    def __repr__(self):
        return f"NDSColor({self.b},{self.g},{self.r},{self.x})"
