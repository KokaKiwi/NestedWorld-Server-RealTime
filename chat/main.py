
def main(args):
    from .ui import ChatApp

    app = ChatApp()
    app.run()


# Entry
from argparse import ArgumentParser

parser = ArgumentParser(prog='nw-chat')
parser.add_argument('-H', '--host', default='127.0.0.1', help='Server host')
parser.add_argument('-p', '--port', type=int, default=6464, help='Server port')


def entry():
    main(parser.parse_args())
