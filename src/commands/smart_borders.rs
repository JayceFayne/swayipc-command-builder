use super::*;

impl Command<SmartBoarders> {
    pub fn on(self) -> Command<Valid> {
        self.push_str("on").transmute()
    }

    pub fn no_gaps(self) -> Command<Valid> {
        self.push_str("no_gaps").transmute()
    }

    pub fn off(self) -> Command<Valid> {
        self.push_str("off").transmute()
    }
}
