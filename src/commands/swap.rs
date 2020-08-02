use super::*;

impl Command<Swap> {
    pub fn with(self) -> Command<Swap<With>> {
        self.push_str("container").push_str("with").transmute()
    }
}

impl Command<Swap<With>> {
    pub fn id(self, id: usize) -> Command<Valid> {
        self.push_str("id").push_str(id.to_string()).transmute()
    }

    pub fn con_id(self, con_id: usize) -> Command<Valid> {
        self.push_str("con_id")
            .push_str(con_id.to_string())
            .transmute()
    }

    pub fn mark(self, mark: impl AsRef<str>) -> Command<Valid> {
        self.push_str("mark").push_str(mark).transmute()
    }
}
