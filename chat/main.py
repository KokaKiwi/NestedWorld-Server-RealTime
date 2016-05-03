import msgpack
import json
import requests
import threading
from collections import defaultdict, namedtuple
from queue import Queue
from prompt_toolkit import prompt


class ChatClient(object):
    API_ROOT = 'http://127.0.0.1:5000/v1'
    API_LOGIN_URL = API_ROOT + '/users/auth/login/simple'

    Listener = namedtuple('Listener', 'test event')

    def __init__(self):
        self.token = None

    def login(self, email=None, password=None):
        if email is None:
            email = prompt('Your e-mail: ')
        if password is None:
            password = prompt('Your password: ', is_password=True)

        data = {
            'app_token': 'test',
            'email': email,
            'password': password,
        }
        data = json.dumps(data)

        headers = {
            'Content-Type': 'application/json',
        }

        r = requests.post(ChatClient.API_LOGIN_URL, data=data, headers=headers)
        r.raise_for_status()

        data = r.json()
        self.token = data['token']

        print('You have been successfully logged in!')

    def logout(self):
        if self.token is None:
            return

    def push(self, msg):
        self.app.push(msg)

    def run(self, host='127.0.0.1', port=6464):
        import socket
        from .ui import ChatApp

        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.connect((host, port))

        self.app = ChatApp()
        self.app.on_message = self._on_ui_line

        def msg_type(ty):
            def test(msg):
                return msg['type'] == ty
            return test

        self.listeners = []
        self._add_listener(msg_type('chat:message-received'), ret=None, event=self._on_message_received)
        self._add_listener(msg_type('chat:user-joined'), ret=None, event=self._on_user_joined)
        t = threading.Thread(target=self._loop, daemon=True)
        t.start()

        app_thread = threading.Thread(target=self.app.run, daemon=True)
        app_thread.start()

        self.channels = defaultdict(list)
        self.join_channel('general')
        self.current_channel = 'general'

        app_thread.join()

    def _loop(self):
        unpacker = msgpack.Unpacker(encoding='utf-8')
        while True:
            buf = self.socket.recv(1024**2)
            unpacker.feed(buf)

            for msg in unpacker:
                self._on_message_raw(msg)

    def _add_listener(self, test_fn, ret=Queue, event=None):
        if ret is not None:
            event = ret()
        self.listeners.append(ChatClient.Listener(test=test_fn, event=event))
        return event

    def _on_ui_line(self, line):
        msg = {
            'type': 'chat:send-message',
            'token': self.token,
            'channel': self.current_channel,
            'message': line.strip(),
        }
        self._send_request(msg)

    def _on_message_raw(self, msg):
        remove_ids = []
        for (i, listener) in enumerate(self.listeners):
            if listener.test(msg):
                if isinstance(listener.event, threading.Event):
                    listener.event.set()
                    remove_ids.append(i)
                elif isinstance(listener.event, Queue):
                    listener.event.put(msg)
                    remove_ids.append(i)
                elif callable(listener.event):
                    if listener.event(msg) is True:
                        remove_ids.append(i)
        for i in reversed(sorted(remove_ids)):
            del self.listeners[i]

    def _on_message_received(self, msg):
        channel = msg['channel']
        user = next(user for user in self.channels[channel] if user['id'] == msg['user'])

        self.push('[{}] {}: {}'.format(channel, user['name'], msg['message']))

    def _on_user_joined(self, msg):
        channel = msg['channel']
        self.channels[channel].append(msg['user'])

    def _send_message_raw(self, msg):
        buf = msgpack.packb(msg)
        self.socket.sendall(buf)

    def _send_request(self, msg):
        import uuid

        req_id = str(uuid.uuid4())

        def test(msg):
            return msg['type'] == 'result' and msg['id'] == req_id
        event = self._add_listener(test)

        msg['id'] = req_id
        self._send_message_raw(msg)

        result = event.get()

        return result

    def join_channel(self, name):
        if self.token is None:
            return

        msg = {
            'type': 'chat:join-channel',
            'token': self.token,
            'channel': name,
        }
        res = self._send_request(msg)
        if res['result'] == 'success':
            self.channels[name] = res['users']
            self.push('You joined the channel `{}`'.format(name))
        else:
            self.push('Error during joining channel `{}`'.format(name))


def main(args):
    client = ChatClient()
    client.login()

    client.run(host=args.host, port=args.port)

    client.logout()


# Entry
from argparse import ArgumentParser

parser = ArgumentParser(prog='nw-chat')
parser.add_argument('-H', '--host', default='127.0.0.1', help='Server host')
parser.add_argument('-p', '--port', type=int, default=6464, help='Server port')


def entry():
    main(parser.parse_args())
