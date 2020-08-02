use super::*;

impl Command<Unbindswitch> {
    pub fn switch(self, switch: impl AsRef<str>) -> Command<Unbindswitch<With>> {
        self.push_str(switch).transmute()
    }

    pub fn locked(self) -> Command<Unbindswitch> {
        self.push_str("--locked").transmute()
    }

    pub fn no_warn(self) -> Command<Unbindswitch> {
        self.push_str("--no-warn").transmute()
    }

    pub fn reload(self) -> Command<Unbindswitch> {
        self.push_str("--reload").transmute()
    }
}

impl Command<Unbindswitch<With>> {
    pub fn state(self, state: impl AsRef<str>) -> Command<Valid> {
        self.push_char(':')
            .push_str_without_space(state)
            .transmute()
    }
}
