use super::*;

impl Command<Opacity> {
    pub fn set(self) -> Command<Opacity<With>> {
        self.push_str("set").transmute()
    }

    pub fn plus(self) -> Command<Opacity<With>> {
        self.push_str("plus").transmute()
    }

    pub fn minus(self) -> Command<Opacity<With>> {
        self.push_str("minus").transmute()
    }
}

impl Command<Opacity<With>> {
    pub fn value(self, value: f32) -> Command<Valid> {
        self.push_str(value.to_string()).transmute()
    }
}
