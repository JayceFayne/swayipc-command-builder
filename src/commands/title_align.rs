use super::*;

impl Command<TitleAlign> {
    pub fn left(self) -> Command<Valid> {
        self.push_str("left").transmute()
    }

    pub fn center(self) -> Command<Valid> {
        self.push_str("center").transmute()
    }

    pub fn right(self) -> Command<Valid> {
        self.push_str("right").transmute()
    }
}
