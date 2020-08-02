use super::*;

impl Command<FloatingSize> {
    pub fn width(self, width: isize) -> Command<FloatingSize<X>> {
        self.push_str(width.to_string()).transmute()
    }
}

impl Command<FloatingSize<X>> {
    pub fn height(self, height: isize) -> Command<Valid> {
        self.push_char('x').push_str(height.to_string()).transmute()
    }
}
