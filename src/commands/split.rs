use super::*;

impl Command<Split> {
    pub fn vertical(self) -> Command<Valid> {
        self.push_char('v').transmute()
    }

    pub fn horizontal(self) -> Command<Valid> {
        self.push_char('h').transmute()
    }

    pub fn toggle(self) -> Command<Valid> {
        self.push_char('t').transmute()
    }
}
