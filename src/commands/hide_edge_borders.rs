use super::*;

//TODO: i3 compat

impl Command<HideEdgeBorders> {
    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }

    pub fn vertical(self) -> Command<Valid> {
        self.push_str("vertical").transmute()
    }

    pub fn horizontal(self) -> Command<Valid> {
        self.push_str("horizontal").transmute()
    }

    pub fn both(self) -> Command<Valid> {
        self.push_str("both").transmute()
    }

    pub fn smart(self) -> Command<Valid> {
        self.push_str("smart").transmute()
    }

    pub fn smart_no_gaps(self) -> Command<Valid> {
        self.push_str("smart_no_gaps").transmute()
    }
}
