import ndspy


class NDSSpriteSheet:
    pass


class NSCR:
    pass


class NCLR:
    '''
    Nintendo DS palette format
    '''

    def __init__(self, data):
        pass


class _NDSColor:
    '''
    A single color in BRG555 format (that is, one color in NTPF)
    '''

    def __init__(self, b, g, r, x=False):
        self.r = r
        self.g = g
        self.b = b
        self.x = x

    def from_bin(bin):
        self = _NDSColor(0, 0, 0)
        col = int.from_bytes(bin, "little")
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
