use super::*;

impl Command<MouseWraping> {
    pub fn off(self) -> Command<Valid> {
        self.push_str("off").transmute()
    }

    pub fn msec(self, msec: usize) -> Command<Valid> {
        self.push_str(msec.to_string()).transmute()
    }
}
