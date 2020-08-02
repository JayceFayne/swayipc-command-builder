use super::*;

impl Command<Rename> {
    pub fn current_workspace(self) -> Command<Rename<To>> {
        self.push_str("workspace").transmute()
    }

    pub fn workspace(self, name: impl AsRef<str>) -> Command<Rename<To>> {
        self.push_str("workspace").push_str(name).transmute()
    }
}

impl Command<Rename<To>> {
    pub fn to(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str("to").push_str(name).transmute()
    }
}
