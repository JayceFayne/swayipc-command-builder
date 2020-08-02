use super::*;

impl Command<Scratchpad> {
    pub fn show(self) -> Command<Valid> {
        self.push_str("show").transmute()
    }
}
