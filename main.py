from dslake.img import _NDSColor

color = _NDSColor.from_rgb888(255, 255, 255, True).to_bin()
print(bin(int.from_bytes(color, "little")))
