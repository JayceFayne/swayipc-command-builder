use super::*;

impl Command<InhibitIdle> {
    pub fn focus(self) -> Command<Valid> {
        self.push_str("focus").transmute()
    }

    pub fn fullscreen(self) -> Command<Valid> {
        self.push_str("fullscreen").transmute()
    }

    pub fn open(self) -> Command<Valid> {
        self.push_str("open").transmute()
    }

    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }

    pub fn visible(self) -> Command<Valid> {
        self.push_str("visible").transmute()
    }
}
