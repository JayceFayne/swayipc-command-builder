use super::*;

impl Command<ShortcutsInhibitor> {
    pub fn enable(self) -> Command<Valid> {
        self.push_str("enable").transmute()
    }

    pub fn disable(self) -> Command<Valid> {
        self.push_str("disable").transmute()
    }
}
