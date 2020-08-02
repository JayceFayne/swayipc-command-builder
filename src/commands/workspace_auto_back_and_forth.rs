use super::*;

impl Command<WorkspaceAutoBackAndForth> {
    pub fn yes(self) -> Command<Valid> {
        self.push_str("yes").transmute()
    }

    pub fn no(self) -> Command<Valid> {
        self.push_str("no").transmute()
    }
}
