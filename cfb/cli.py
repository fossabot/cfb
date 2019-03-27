"""Generate code from serialized flatbuffers schema in bfbs format.

Usage:
  cfbc [-o <dir>] <bfbs>
  cfbc -h | --help
  cfbc --version

Options:
  -o <dir>   Save all generated files in <dir>, instead of the directory containing <bfbs>.
  <bfbs>     Load schema from <bfbs> which is generated by `flatc -b --schema <fbs>`.

  -h --help  Show this screen.
  --version  Show version.
"""
from docopt import docopt
from cfb.generator import Generator
from cfb.version import VERSION


def parse_arguments(argv=None):
    return docopt(__doc__, argv)


def generate(arguments):
    g = Generator(arguments['<bfbs>'])
    g.generate(arguments['-o'])


def main():
    arguments = parse_arguments()
    if arguments['--version']:
        print(VERSION)
    else:
        generate(arguments)
