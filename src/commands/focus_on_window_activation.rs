use super::*;

impl Command<FocusOnWindowActivation> {
    pub fn smart(self) -> Command<Valid> {
        self.push_str("smart").transmute()
    }

    pub fn urgent(self) -> Command<Valid> {
        self.push_str("urgent").transmute()
    }

    pub fn focus(self) -> Command<Valid> {
        self.push_str("focus").transmute()
    }

    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }
}
