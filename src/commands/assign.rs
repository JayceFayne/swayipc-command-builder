use super::*;

impl Command<Assign> {
    pub fn workspace(self) -> Command<Assign<Workspace>> {
        self.push_str("workspace").transmute()
    }

    pub fn output(self) -> Command<Assign<Output>> {
        self.push_str("output").transmute()
    }
}

impl Command<Assign<Workspace>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }

    pub fn number(self, number: isize) -> Command<Valid> {
        self.push_str("number")
            .push_str(number.to_string())
            .transmute()
    }
}

impl Command<Assign<Output>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }

    pub fn up(self) -> Command<Valid> {
        self.push_str("up").transmute()
    }

    pub fn right(self) -> Command<Valid> {
        self.push_str("right").transmute()
    }

    pub fn down(self) -> Command<Valid> {
        self.push_str("down").transmute()
    }

    pub fn left(self) -> Command<Valid> {
        self.push_str("left").transmute()
    }
}
