use super::*;

impl Command<Unmark> {
    pub fn all_identifier(self) -> Command<Valid> {
        self.transmute()
    }

    pub fn identifier(self, identifier: impl AsRef<str>) -> Command<Valid> {
        self.push_str(identifier).transmute()
    }
}
