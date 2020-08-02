use super::*;

impl Command<MouseWraping> {
    pub fn output(self) -> Command<Valid> {
        self.push_str("output").transmute()
    }

    pub fn container(self) -> Command<Valid> {
        self.push_str("container").transmute()
    }

    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }
}
