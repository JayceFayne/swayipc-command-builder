use super::*;

impl Command<Border> {
    pub fn csd(self) -> Command<Valid> {
        self.push_str("csd").transmute()
    }

    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }

    pub fn toggle(self) -> Command<Valid> {
        self.push_str("toggle").transmute()
    }

    pub fn normal(self) -> Command<Valid<Border<With>>> {
        self.push_str("normal").transmute()
    }

    pub fn pixel(self) -> Command<Valid<Border<With>>> {
        self.push_str("pixel").transmute()
    }
}

impl Command<Valid<Border<With>>> {
    pub fn with(self, n: isize) -> Command<Valid> {
        self.push_str(n.to_string()).transmute()
    }
}
