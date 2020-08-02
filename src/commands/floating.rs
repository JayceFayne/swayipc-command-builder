use super::*;

impl Command<Floating> {
    pub fn enable(self) -> Command<Valid> {
        self.push_str("enable").transmute()
    }

    pub fn disable(self) -> Command<Valid> {
        self.push_str("disable").transmute()
    }

    pub fn toggle(self) -> Command<Valid> {
        self.push_str("toggle").transmute()
    }
}
