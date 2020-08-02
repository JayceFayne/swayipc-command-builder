use super::*;

impl Command<DefaultBorder> {
    pub fn normal(self) -> Command<Valid> {
        self.push_str("normal").transmute()
    }

    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }

    pub fn pixel(self, px: usize) -> Command<Valid> {
        self.push_str("pixel").push_str(px.to_string()).transmute()
    }
}
