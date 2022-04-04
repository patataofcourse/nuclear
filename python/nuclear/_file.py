import ndspy


class SectionedDSFile:
    def __init__(self, magic, sections):
        self.magic = magic
        self.sections = sections


class Section:
    def __init__(self, magic, content):
        self.magic = magic
        self.content = content


def parse_header(data):
    '''
    Parse a generic Nintendo DS file header, and divide the file in sections
    '''

    # Header

    magic = str(data[:4], encoding="ascii")
    # 4:6 should always be 0xFFFE in whichever endian (BOM) #TODO: manage Big Endian
    # 6:8 is apparently always 0x01
    # 8:0xC should be full filesize
    # 0xC:0xE should always be 0x10
    num_sections = int.from_bytes(data[0xE:0x10], "little")

    # Sections

    pos = 0x10
    section_count = 0
    sections = []
    while section_count < num_sections:
        magic_ = str(data[pos:pos+4], "ascii")
        size = int.from_bytes(data[pos+4:pos+8], "little")
        content = data[pos+8:pos+size]
        sections.append(Section(magic_, content))
        pos += size
        section_count += 1

    return SectionedDSFile(magic, sections)


def build_file(magic, sections, endian="little"):
    pass
