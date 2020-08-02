use super::*;

impl Command<Workspace> {
    pub fn no_auto_back_and_forth(self) -> Command<Workspace<With>> {
        self.push_str("--no-auto-back-and-forth").transmute()
    }

    pub fn goto(self) -> Command<Workspace<With>> {
        self.transmute()
    }

    pub fn next(self) -> Command<Valid> {
        self.push_str("next").transmute()
    }

    pub fn prev(self) -> Command<Valid> {
        self.push_str("prev").transmute()
    }

    pub fn next_on_output(self) -> Command<Valid> {
        self.push_str("next_on_output").transmute()
    }

    pub fn prev_on_output(self) -> Command<Valid> {
        self.push_str("prev_on_output").transmute()
    }

    pub fn back_and_forth(self) -> Command<Valid> {
        self.push_str("back_and_forth").transmute()
    }

    pub fn name(self, name: impl AsRef<str>) -> Command<Workspace<X>> {
        self.push_str(name).transmute()
    }
}

impl Command<Workspace<With>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }

    pub fn number(self, number: isize) -> Command<Valid> {
        self.push_str(number.to_string()).transmute()
    }
}

impl Command<Workspace<X>> {
    pub fn gaps(self) -> Command<Workspace<X<Gaps>>> {
        self.transmute()
    }

    pub fn output(self, output: impl AsRef<str>) -> Command<Valid> {
        self.push_str(output).transmute()
    }
}

impl Command<Workspace<X<Gaps>>> {
    pub fn inner(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("inner").transmute()
    }

    pub fn outer(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("outer").transmute()
    }

    pub fn horizontal(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("horizontal").transmute()
    }

    pub fn vertical(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("vertical").transmute()
    }

    pub fn top(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("top").transmute()
    }

    pub fn right(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("right").transmute()
    }

    pub fn bottom(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("bottom").transmute()
    }

    pub fn left(self) -> Command<Workspace<X<Gaps<By>>>> {
        self.push_str("left").transmute()
    }
}

impl Command<Workspace<X<Gaps<By>>>> {
    pub fn amount(self, amount: usize) -> Command<Valid> {
        self.push_str(amount.to_string()).transmute()
    }
}
