use super::*;

impl Command<PopupDuringFullscreen> {
    pub fn smart(self) -> Command<Valid> {
        self.push_str("smart").transmute()
    }

    pub fn ignore(self) -> Command<Valid> {
        self.push_str("ignore").transmute()
    }

    pub fn leave_fullscreen(self) -> Command<Valid> {
        self.push_str("leave_fullscreen").transmute()
    }
}
