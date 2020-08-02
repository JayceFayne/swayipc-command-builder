use super::*;

impl Command<Gaps> {
    pub fn inner(self) -> Command<Gaps<Select>> {
        self.push_str("inner").transmute()
    }

    pub fn outer(self) -> Command<Gaps<Select>> {
        self.push_str("outer").transmute()
    }

    pub fn horizontal(self) -> Command<Gaps<Select>> {
        self.push_str("horizontal").transmute()
    }

    pub fn vertical(self) -> Command<Gaps<Select>> {
        self.push_str("vertical").transmute()
    }

    pub fn top(self) -> Command<Gaps<Select>> {
        self.push_str("top").transmute()
    }

    pub fn right(self) -> Command<Gaps<Select>> {
        self.push_str("right").transmute()
    }

    pub fn bottom(self) -> Command<Gaps<Select>> {
        self.push_str("bottom").transmute()
    }

    pub fn left(self) -> Command<Gaps<Select>> {
        self.push_str("left").transmute()
    }
}

impl Command<Gaps<Select>> {
    pub fn all(self) -> Command<Gaps<Select<With>>> {
        self.push_str("all").transmute()
    }

    pub fn current(self) -> Command<Gaps<Select<With>>> {
        self.push_str("current").transmute()
    }
}

impl Command<Gaps<Select<With>>> {
    pub fn set(self) -> Command<Gaps<Select<With<X>>>> {
        self.push_str("set").transmute()
    }

    pub fn plus(self) -> Command<Gaps<Select<With<X>>>> {
        self.push_str("plus").transmute()
    }

    pub fn minus(self) -> Command<Gaps<Select<With<X>>>> {
        self.push_str("minus").transmute()
    }
}

impl Command<Gaps<Select<With<X>>>> {
    pub fn amount(self, amount: usize) -> Command<Valid> {
        self.push_str(amount.to_string()).transmute()
    }
}
