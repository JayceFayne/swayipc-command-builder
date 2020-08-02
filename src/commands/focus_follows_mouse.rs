use super::*;

impl Command<FocusFollowMouse> {
    pub fn yes(self) -> Command<Valid> {
        self.push_str("yes").transmute()
    }

    pub fn no(self) -> Command<Valid> {
        self.push_str("no").transmute()
    }

    pub fn always(self) -> Command<Valid> {
        self.push_str("always").transmute()
    }
}
