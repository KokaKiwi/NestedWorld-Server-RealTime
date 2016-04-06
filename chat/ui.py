import npyscreen as nps


class ChatApp(nps.NPSAppManaged):

    def onStart(self):
        self.registerForm('MAIN', IntroForm())
        self.registerForm('CHAT', ChatForm())


class IntroForm(nps.Popup):

    def __init__(self):
        super().__init__(name='Intro')

    def create(self):
        self.name_field = self.add(nps.TitleText, name='Name:')

    def afterEditing(self):
        self.parentApp.setNextForm('CHAT')


class ChatForm(nps.FormMutt):

    def create(self):
        super().create()

        self.add_handlers({'^D': lambda *args: self.parentApp.switchForm(None)})
