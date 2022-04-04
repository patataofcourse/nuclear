from . import pal

def setup_cli(group):
    group.add_command(pal.pal, "pal")