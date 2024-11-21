//! components to test inserting and fetching

use crate::treecs::{EntityKey, Treecs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]

pub struct Name {
    pub name: &'static str,
}
impl Name {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

pub trait Animal {
    fn noise(&self) -> &'static str;
}
pub struct Dog;
impl Animal for Dog {
    fn noise(&self) -> &'static str {
        "Bark"
    }
}
pub struct Cat;
impl Animal for Cat {
    fn noise(&self) -> &'static str {
        "Meow"
    }
}
