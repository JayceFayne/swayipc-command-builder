use super::*;

impl Command<Fullscreen> {
    pub fn enable(self) -> Command<Valid<Fullscreen>> {
        self.push_str("enable").transmute()
    }

    pub fn disable(self) -> Command<Valid<Fullscreen>> {
        self.push_str("disable").transmute()
    }

    pub fn toggle(self) -> Command<Valid<Fullscreen>> {
        self.push_str("toggle").transmute()
    }
}

impl Command<Valid<Fullscreen>> {
    pub fn global(self) -> Command<Valid> {
        self.push_str("global").transmute()
    }
}
