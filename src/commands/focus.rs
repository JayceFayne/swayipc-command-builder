use super::*;

impl Command<Valid<Focus>> {
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

    pub fn prev(self) -> Command<Valid<Focus<With>>> {
        self.push_str("prev").transmute()
    }

    pub fn next(self) -> Command<Valid<Focus<With>>> {
        self.push_str("next").transmute()
    }

    pub fn child(self) -> Command<Valid> {
        self.push_str("child").transmute()
    }

    pub fn parent(self) -> Command<Valid> {
        self.push_str("parent").transmute()
    }

    pub fn output(self) -> Command<Focus<Output>> {
        self.push_str("output").transmute()
    }

    //FIXME: sway(5)
    pub fn tiling(self) -> Command<Valid> {
        self.push_str("tiling").transmute()
    }

    //FIXME: sway(5)
    pub fn mode_toggle(self) -> Command<Valid> {
        self.push_str("mode_toggle").transmute()
    }

    pub fn floating(self) -> Command<Valid> {
        self.push_str("floating").transmute()
    }
}

impl Command<Valid<Focus<With>>> {
    pub fn sibling(self) -> Command<Valid> {
        self.push_str("sibling").transmute()
    }
}

impl Command<Valid<Focus<Output>>> {
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
