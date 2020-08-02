use super::*;

impl Command<Layout> {
    pub fn default(self) -> Command<Valid> {
        self.push_str("default").transmute()
    }

    pub fn splith(self) -> Command<Valid> {
        self.push_str("splith").transmute()
    }

    pub fn splitv(self) -> Command<Valid> {
        self.push_str("splitv").transmute()
    }

    pub fn stacking(self) -> Command<Valid> {
        self.push_str("stacking").transmute()
    }

    pub fn tabbed(self) -> Command<Valid> {
        self.push_str("tabbed").transmute()
    }

    pub fn toggle(self) -> Command<Layout<X>> {
        self.push_str("toggle").transmute()
    }
}

impl Command<Layout<X>> {
    pub fn split(self) -> Command<Valid> {
        self.push_str("split").transmute()
    }

    pub fn all(self) -> Command<Valid> {
        self.push_str("all").transmute()
    }

    pub fn through(self, list: impl AsRef<str>) -> Command<Valid> {
        self.push_str(list).transmute()
    }
}
