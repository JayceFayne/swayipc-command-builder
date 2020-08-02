use super::*;

impl Command<Resize> {
    pub fn shrink(self) -> Command<Resize<Direction>> {
        self.push_str("shrink").transmute()
    }

    pub fn grow(self) -> Command<Resize<Direction>> {
        self.push_str("grow").transmute()
    }

    //FIXME: sway(5)
    pub fn set(self) -> Command<Resize<With>> {
        self.push_str("set").transmute()
    }
}

impl Command<Resize<With>> {
    pub fn height(self) -> Command<Resize<With<To>>> {
        self.push_str("height").transmute()
    }

    pub fn width(self) -> Command<Resize<With<To>>> {
        self.push_str("width").transmute()
    }
}

impl Command<Resize<With<To>>> {
    pub fn to(self, val: usize) -> Command<Valid<Resize<Direction<With>>>> {
        self.push_str(val.to_string()).transmute()
    }
}

impl Command<Resize<Direction>> {
    pub fn width(self) -> Command<Valid<Resize<Direction>>> {
        self.push_str("width").transmute()
    }

    pub fn height(self) -> Command<Valid> {
        self.push_str("height").transmute()
    }
}

impl Command<Valid<Resize<Direction>>> {
    pub fn amount(self, amount: usize) -> Command<Valid<Resize<Direction<With>>>> {
        self.push_str(amount.to_string()).transmute()
    }
}

impl Command<Valid<Resize<Direction<With>>>> {
    pub fn px(self) -> Command<Valid> {
        self.push_str("px").transmute()
    }

    pub fn ppt(self) -> Command<Valid> {
        self.push_str("ppt").transmute()
    }
}
