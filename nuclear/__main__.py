import click
from .cli import setup_cli

@click.group(name="nuclear", options_metavar="")
def cli():
    pass

setup_cli(cli)
cli()