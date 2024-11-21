use crate::treecs::{query::QueryAble, EntityKey, Treecs};

use super::QueryIter;

/// iteratore over the parents of the nodes
pub struct ParentIter<'world> {
    world: &'world Treecs,
    current_key: Option<EntityKey>,
}
impl<'world> Iterator for ParentIter<'world> {
    type Item = EntityKey;
    fn next(&mut self) -> Option<Self::Item> {
        let curr_key = self.current_key?;
        let linkdata = self.world.linkdata(curr_key)?;
        self.current_key = *linkdata.parent();
        Some(curr_key)
    }
}
impl<'world, Q: QueryAble> QueryIter<'world, Q> for ParentIter<'world> {
    type Info = EntityKey;
    fn transform(
        &self,
        key: EntityKey,
    ) -> Option<Q::Output<'world>> {
        Q::get(&self.world, key)
    }
}
impl<'world> ParentIter<'world> {
    pub fn new(world: &'world Treecs, start: EntityKey) -> Self {
        Self {
            world,
            current_key: Some(start),
        }
    }
}
