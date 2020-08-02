use super::states::Valid;
use std::default::Default;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

//TODO: document behavior on multiple fn call like `Filter::new().shell("a").shell("b")`

pub struct Filter<T = ()> {
    inner: String,
    state: PhantomData<T>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
            state: PhantomData,
        }
    }
}

impl<T> Filter<T> {
    fn insert<N>(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Filter<N> {
        let mut inner = self.inner;
        if inner.is_empty() {
            inner.push('[');
        } else {
            inner.pop();
            inner.push(' ');
        }
        inner.push_str(key.as_ref());
        inner.push('=');
        inner.push_str(value.as_ref());
        inner.push(']');
        Filter {
            inner,
            state: PhantomData,
        }
    }
}

pub trait Finalize {}
impl Finalize for () {}
impl Finalize for Valid {}

impl<T: Finalize> Filter<T> {
    pub fn app_id(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("app_id", value)
    }

    pub fn class(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("class", value)
    }

    pub fn con_id(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("con_id", value)
    }

    pub fn con_mark(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("con_mark", value)
    }

    pub fn floating(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("floating", value)
    }

    pub fn id(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("id", value)
    }

    pub fn instance(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("instance", value)
    }

    pub fn pid(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("pid", value)
    }

    pub fn shell(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("shell", value)
    }

    pub fn tiling(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("tiling", value)
    }

    pub fn title(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("title", value)
    }

    pub fn urgent(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("urgent", value)
    }

    pub fn window_role(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("window_role", value)
    }

    pub fn window_type(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("window_type", value)
    }

    pub fn workspace(self, value: impl AsRef<str>) -> Filter<Valid> {
        self.insert("workspace", value)
    }
}

impl<T> Debug for Filter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for Filter<Valid> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl AsRef<str> for Filter<Valid> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl From<Filter<Valid>> for String {
    fn from(filter: Filter<Valid>) -> Self {
        filter.inner
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
