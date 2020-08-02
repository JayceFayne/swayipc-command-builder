use super::*;

impl Command<Move> {
    pub fn left(self) -> Command<Valid<Move<Direction>>> {
        self.push_str("left").transmute()
    }

    pub fn right(self) -> Command<Valid<Move<Direction>>> {
        self.push_str("right").transmute()
    }

    pub fn up(self) -> Command<Valid<Move<Direction>>> {
        self.push_str("up").transmute()
    }

    pub fn down(self) -> Command<Valid<Move<Direction>>> {
        self.push_str("down").transmute()
    }

    pub fn absolute(self) -> Command<Valid<Move<Direction>>> {
        self.push_str("absolute").transmute()
    }

    pub fn no_auto_back_and_forth(self) -> Command<Move<NoAuto>> {
        self.push_str("--no-auto-back-and-forth").transmute()
    }

    pub fn position(self) -> Command<Move<Position>> {
        self.push_str("position").transmute()
    }

    pub fn window(self) -> Command<Move<WinCon>> {
        self.push_str("window").transmute()
    }

    pub fn container(self) -> Command<Move<WinCon>> {
        self.push_str("container").transmute()
    }

    pub fn workspace(self) -> Command<Move<Workspace>> {
        self.push_str("workspace").transmute()
    }
}

impl Command<Move<Workspace>> {
    pub fn to(self) -> Command<Move<Workspace<To>>> {
        self.push_str("to").transmute()
    }
}

impl Command<Move<Workspace<To>>> {
    pub fn output(self) -> Command<Move<WinCon<To<Output>>>> {
        self.push_str("output").transmute()
    }
}

impl Command<Move<Absolute>> {
    pub fn position(self) -> Command<Move<Absolute<Position>>> {
        self.push_str("position").transmute()
    }
}

impl Command<Move<Absolute<Position>>> {
    pub fn pos_x(self, x: usize) -> Command<Move<Absolute<Position<X<With>>>>> {
        self.push_str(x.to_string()).transmute()
    }

    pub fn center(self) -> Command<Valid> {
        self.push_str("center").transmute()
    }
}

impl Command<Move<Absolute<Position<X<With>>>>> {
    pub fn in_px(self) -> Command<Move<Absolute<Position<X<With<Y>>>>>> {
        self.push_str("px").transmute()
    }

    pub fn in_ppt(self) -> Command<Move<Absolute<Position<X<With<Y>>>>>> {
        self.push_str("ppt").transmute()
    }
}

impl Command<Move<Absolute<Position<X<With<Y>>>>>> {
    pub fn pos_y(self, y: usize) -> Command<Move<Absolute<Position<X<With<Y<With>>>>>>> {
        self.push_str(y.to_string()).transmute()
    }
}

impl Command<Move<Absolute<Position<X<With<Y<With>>>>>>> {
    pub fn in_px(self) -> Command<Valid> {
        self.push_str("px").transmute()
    }

    pub fn in_ppt(self) -> Command<Valid> {
        self.push_str("ppt").transmute()
    }
}

impl Command<Move<Position>> {
    pub fn pos_x(self, x: usize) -> Command<Move<Absolute<Position<X<With>>>>> {
        self.push_str(x.to_string()).transmute()
    }

    pub fn center(self) -> Command<Valid> {
        self.push_str("center").transmute()
    }

    pub fn mouse(self) -> Command<Valid> {
        self.push_str("mouse").transmute()
    }

    pub fn cursor(self) -> Command<Valid> {
        self.push_str("cursor").transmute()
    }

    pub fn pointer(self) -> Command<Valid> {
        self.push_str("pointer").transmute()
    }
}

impl Command<Valid<Move<Direction>>> {
    pub fn by(self, px: usize) -> Command<Valid> {
        self.push_str(px.to_string()).push_str("px").transmute()
    }
}

impl Command<Move<NoAuto>> {
    pub fn window(self) -> Command<Move<NoAuto<WinCon>>> {
        self.push_str("window").transmute()
    }

    pub fn container(self) -> Command<Move<NoAuto<WinCon>>> {
        self.push_str("container").transmute()
    }
}

impl Command<Move<NoAuto<WinCon>>> {
    pub fn to(self) -> Command<Move<WinCon<NoAuto<To>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<NoAuto<To>>>> {
    pub fn workspace(self) -> Command<Move<WinCon<NoAuto<To<Workspace>>>>> {
        self.push_str("workspace").transmute()
    }
}

impl Command<Move<WinCon<NoAuto<To<Workspace>>>>> {
    pub fn with(self) -> Command<Move<WinCon<To<Workspace<To<With>>>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<To<Workspace>>>> {
    pub fn with(self) -> Command<Move<WinCon<To<Workspace<To<With>>>>>> {
        self.transmute()
    }

    pub fn current(self) -> Command<Valid> {
        self.push_str("current").transmute()
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
}

impl Command<Move<WinCon<To<Workspace<To<With>>>>>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }

    pub fn number(self, id: usize) -> Command<Valid> {
        self.push_str("number").push_str(id.to_string()).transmute()
    }
}

impl Command<Move<WinCon>> {
    pub fn to(self) -> Command<Move<WinCon<To>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<To>>> {
    pub fn mark(self, mark: impl AsRef<str>) -> Command<Valid> {
        self.push_str("mark").push_str(mark).transmute()
    }

    pub fn scratchpad(self) -> Command<Valid> {
        self.push_str("scratchpad").transmute()
    }

    pub fn workspace(self) -> Command<Move<WinCon<To<Workspace>>>> {
        self.push_str("workspace").transmute()
    }

    pub fn output(self) -> Command<Move<WinCon<To<Output>>>> {
        self.push_str("output").transmute()
    }
}

impl Command<Move<WinCon<To<Output>>>> {
    pub fn left(self) -> Command<Valid> {
        self.push_str("left").transmute()
    }

    pub fn right(self) -> Command<Valid> {
        self.push_str("right").transmute()
    }

    pub fn up(self) -> Command<Valid> {
        self.push_str("up").transmute()
    }

    pub fn down(self) -> Command<Valid> {
        self.push_str("down").transmute()
    }

    pub fn current(self) -> Command<Move<WinCon<To<Output>>>> {
        self.push_str("current").transmute()
    }

    pub fn with(self) -> Command<Move<WinCon<To<Output<With>>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<To<Output<With>>>>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Valid> {
        self.push_str(name).transmute()
    }

    pub fn id(self, id: usize) -> Command<Valid> {
        self.push_str(id.to_string()).transmute()
    }
}
