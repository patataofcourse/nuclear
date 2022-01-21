import click
from nuclear import img
import os

@click.group(help="NCLR/palette format", options_metavar="")
def pal():
    pass

@pal.command(
                name = "extract",
                help= "Extracts a palette into a folder of PNG files",
                no_args_is_help = True
            )
@click.argument("in")
@click.argument("out")
def extract(**kwargs):
    in_ = kwargs["in"]
    out = kwargs["out"]

    nclr = img.NCLR(open(in_, "rb").read())
    outfiles = nclr.export()
    try:
        os.mkdir(out)
    except:
        print("Warning: target directory already exists")
    for file in outfiles:
        o = open(f"{out}/{file}", "wb")
        o.write(outfiles[file])
        o.close()
