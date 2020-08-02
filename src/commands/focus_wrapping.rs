use super::*;

impl Command<FocusWrapping> {
    pub fn yes(self) -> Command<Valid> {
        self.push_str("yes").transmute()
    }

    pub fn no(self) -> Command<Valid> {
        self.push_str("no").transmute()
    }

    pub fn force(self) -> Command<Valid> {
        self.push_str("force").transmute()
    }

    pub fn workspace(self) -> Command<Valid> {
        self.push_str("workspace").transmute()
    }
}
