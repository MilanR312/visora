use crate::treecs::{component::Component, query::QueryAble, EntityKey, Treecs};




pub trait InfoTransform<'world, Q: QueryAble>{
    type Output;
    fn transform(self, data: Q::Output<'world>) -> Self::Output;
    fn get_key(&self) -> EntityKey;
}
impl<'world, Q: QueryAble> InfoTransform<'world, Q> for EntityKey {
    type Output = <Q as QueryAble>::Output<'world>;
    fn transform(self, data: <Q as QueryAble>::Output<'world>) -> Self::Output {
        data
    }
    fn get_key(&self) -> EntityKey {
        *self
    }
}
impl<'world, T, Q: QueryAble> InfoTransform<'world, Q> for (T, EntityKey){
    type Output = (T, <Q as QueryAble>::Output<'world>);
    fn transform(self, data: <Q as QueryAble>::Output<'world>) -> Self::Output {
        (self.0, data)
    }
    fn get_key(&self) -> EntityKey {
        self.1
    }
}
pub trait QueryIter<'world, Q: QueryAble>: Iterator<Item = Self::Info> {
    type Info: InfoTransform<'world, Q>;
    fn transform(&self, key: EntityKey) -> Option<Q::Output<'world>>;
}

/// an iterator that can created from only the world
pub trait WorldIter<'world, Q: QueryAble>: QueryIter<'world, Q> {
    fn new(world: &'world Treecs) -> Self;
    fn restart(self) -> Self;
}

pub mod breadth;
pub mod parent;
