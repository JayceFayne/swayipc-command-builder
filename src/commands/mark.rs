use super::*;

impl Command<Mark> {
    pub fn add(self) -> Command<Mark<Add>> {
        self.push_str("--add").transmute()
    }

    pub fn replace(self) -> Command<Mark<Add>> {
        self.push_str("--replace").transmute()
    }
}

impl Command<Mark<Add>> {
    pub fn toggle(self) -> Command<Mark<Add<Replace>>> {
        self.push_str("--toggle").transmute()
    }

    pub fn identifier(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }
}

impl Command<Mark<Add<Replace>>> {
    pub fn identifier(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }
}
