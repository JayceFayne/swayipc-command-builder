use super::*;

impl Command<Client> {
    pub fn background(self) -> Command<Client<I3>> {
        self.push_str_without_space("background").transmute()
    }

    pub fn focused(self) -> Command<Client<With>> {
        self.push_str_without_space("focused").transmute()
    }

    pub fn focused_inactive(self) -> Command<Client<With>> {
        self.push_str_without_space("focused_inactive").transmute()
    }

    pub fn placeholder(self) -> Command<Client<With>> {
        self.push_str_without_space("placeholder").transmute()
    }

    pub fn unfocused(self) -> Command<Client<With>> {
        self.push_str_without_space("unfocused").transmute()
    }

    pub fn urgent(self) -> Command<Client<With>> {
        self.push_str_without_space("urgent").transmute()
    }
}

impl Command<Client<I3>> {
    pub fn color(self, val: impl AsRef<str>) -> Command<Valid> {
        self.push_str(val).transmute()
    }
}

impl Command<Client<With>> {
    pub fn border(self, color: impl AsRef<str>) -> Command<Client<Border>> {
        self.push_str(color).transmute()
    }
}

impl Command<Client<Border>> {
    pub fn background(self, color: impl AsRef<str>) -> Command<Client<Background>> {
        self.push_str(color).transmute()
    }
}

impl Command<Client<Background>> {
    pub fn text(self, color: impl AsRef<str>) -> Command<Valid<Client<Text>>> {
        self.push_str(color).transmute()
    }
}

impl Command<Valid<Client<Text>>> {
    pub fn indicator(self, color: impl AsRef<str>) -> Command<Valid<Client<Indicator>>> {
        self.push_str(color).transmute()
    }
}

impl Command<Valid<Client<Indicator>>> {
    pub fn child_border(self, color: impl AsRef<str>) -> Command<Valid> {
        self.push_str(color).transmute()
    }
}
