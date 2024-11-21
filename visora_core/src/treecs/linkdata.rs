use crate::treecs::{children::Children, EntityKey};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkData {
    parent: Option<EntityKey>,
    children: Children<EntityKey>,
}

impl LinkData {
    const UNCONNECTED: LinkData = const {
        LinkData {
            parent: None,
            children: Children::NoChild,
        }
    };

    pub fn new_empty() -> Self {
        Self::UNCONNECTED.clone()
    }
    pub fn new_with_parent(parent: EntityKey) -> Self {
        Self {
            parent: Some(parent),
            children: Children::NoChild,
        }
    }

    pub fn children(&self) -> &Children<EntityKey> {
        &self.children
    }
    pub fn children_mut(&mut self) -> &mut Children<EntityKey> {
        &mut self.children
    }
    pub fn parent(&self) -> &Option<EntityKey> {
        &self.parent
    }
    pub fn parent_mut(&mut self) -> &mut Option<EntityKey> {
        &mut self.parent
    }
}
