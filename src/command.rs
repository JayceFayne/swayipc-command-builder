use super::states::{And, Chain, Filter, ForWindow, Valid};
use std::default::Default;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

//TODO: macro

pub trait Finalize {}
impl Finalize for () {}
impl Finalize for ForWindow {}
impl Finalize for Filter {}
impl Finalize for And {}
impl Finalize for Chain {}

//FIXME: better trait name
pub trait Chained {}
impl Chained for () {}
impl Chained for And {}
impl Chained for Chain {}

pub trait AddFilter {}
impl AddFilter for () {}
impl AddFilter for Chain {}

pub struct Command<T = ()> {
    inner: String,
    state: PhantomData<T>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
            state: PhantomData,
        }
    }
}

impl Command<Valid> {
    pub fn new_unchecked(inner: impl Into<String>) -> Self {
        Self {
            inner: inner.into(),
            state: PhantomData,
        }
    }
}

impl<T> Command<Valid<T>> {
    pub fn and(self) -> Command<And> {
        self.push_char(',').transmute()
    }

    pub fn chain(self) -> Command<Chain> {
        self.push_char(';').transmute()
    }
}

impl<T> Command<T> {
    pub(super) fn push_str(mut self, val: impl AsRef<str>) -> Self {
        if !self.inner.is_empty() {
            self.inner.push(' ');
        }
        self.push_str_without_space(val)
    }

    pub(super) fn push_str_without_space(mut self, val: impl AsRef<str>) -> Self {
        self.inner.push_str(val.as_ref());
        self
    }

    pub(super) fn push_char(mut self, ch: char) -> Self {
        self.inner.push(ch);
        self
    }

    pub(super) fn transmute<N>(self) -> Command<N> {
        Command {
            inner: self.inner,
            state: PhantomData,
        }
    }
}

impl<T> Debug for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<T> Display for Command<Valid<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<T> AsRef<str> for Command<Valid<T>> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl<T> From<Command<Valid<T>>> for String {
    fn from(command: Command<Valid<T>>) -> Self {
        command.inner
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}
