use super::*;

impl Command<SmartGaps> {
    pub fn on(self) -> Command<Valid> {
        self.push_str("on").transmute()
    }

    pub fn off(self) -> Command<Valid> {
        self.push_str("off").transmute()
    }
}
