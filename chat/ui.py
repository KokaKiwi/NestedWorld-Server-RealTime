import readline
import sys


class ChatApp:

    def __init__(self):
        self.prompt = '> '
        self.on_message = None
        self.has_prompt = False

    def run(self):
        while True:
            self.has_prompt = True
            line = input(self.prompt)
            self.has_prompt = False

            if self.on_message is not None:
                self.on_message(line)

    def push(self, line):
        buf = readline.get_line_buffer()

        # Clean line
        spaces = ' ' * (len(buf) + 2)
        sys.stdout.write('\r' + spaces + '\r')

        # Print line
        print(line, file=sys.stdout)

        # Rewrite input
        if self.has_prompt:
            sys.stdout.write(self.prompt + buf)
        sys.stdout.flush()
