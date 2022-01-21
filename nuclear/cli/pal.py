import click
from nuclear import img

@click.group(help="NCLR/palette format", options_metavar="")
def pal():
    pass

@pal.command(
                name = "extract",
                help= "Extracts a palette into an easily editable PNG file",
                no_args_is_help = True
            )
@click.argument("in")
@click.argument("out")
def extract(**kwargs):
    in_ = kwargs["in"]
    out = kwargs["out"]

    nclr = img.NCLR(open(in_, "rb").read())
