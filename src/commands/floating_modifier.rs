use super::*;

impl Command<FloatingModifier> {
    pub fn none(self) -> Command<Valid> {
        self.push_str("none").transmute()
    }

    pub fn with(self, modifier: impl AsRef<str>) -> Command<FloatingModifier<With>> {
        self.push_str(modifier).transmute()
    }
}

impl Command<FloatingModifier<With>> {
    pub fn normal(self) -> Command<Valid> {
        self.push_str("normal").transmute()
    }

    pub fn inverse(self) -> Command<Valid> {
        self.push_str("inverse").transmute()
    }
}
