use super::*;

impl Command<Urgent> {
    pub fn enable(self) -> Command<Valid> {
        self.push_str("enable").transmute()
    }

    pub fn disable(self) -> Command<Valid> {
        self.push_str("disable").transmute()
    }

    pub fn allow(self) -> Command<Valid> {
        self.push_str("allow").transmute()
    }

    pub fn deny(self) -> Command<Valid> {
        self.push_str("deny").transmute()
    }
}
