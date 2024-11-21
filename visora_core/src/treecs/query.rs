use std::marker::PhantomData;

use crate::treecs::{
    component::Component,
    iterators::{parent::ParentIter, QueryIter, WorldIter},
    EntityKey, Treecs,
};

use super::{component::ComponentEntry, iterators::{breadth::{BreadthInfo, BreadthIter, Forward, Reversed}, InfoTransform}};


pub trait QueryAble {
    type Output<'world>;
    fn get<'world>(world: &'world Treecs, entity: EntityKey) -> Option<Self::Output<'world>>;
}

/*impl<T: Component> QueryAble for (T,) {
    type Output<'world> = &'world T;
    fn get<'world>(world: &'world World, entity: EntityKey) -> Option<Self::Output<'world>> {
        world.get_component::<T>(entity)
    }
}
impl<A: Component, B: Component> QueryAble for (A, B){
    type Output<'world> = (&'world A, &'world B);
    fn get<'world>(world: &'world World, entity: EntityKey) -> Option<Self::Output<'world>> {
        let a = world.get_component::<A>(entity)?;
        let b = world.get_component::<B>(entity)?;
        Some((a,b))
    }
}*/
impl<T: Component> QueryAble for &T {
    type Output<'world> = &'world T;
    //type Output<'world> = &'world T;
    fn get<'world>(world: &'world Treecs, entity: EntityKey) -> Option<Self::Output<'world>> {
        world.get_component(entity)
    }
}
impl<T: Component> QueryAble for Option<&T> {
    type Output<'world> = Option<&'world T>;
    //type Output<'world> = Option<&'world T>;
    fn get<'world>(world: &'world Treecs, entity: EntityKey) -> Option<Self::Output<'world>> {
        Some(world.get_component(entity))
    }
}
impl<A: QueryAble> QueryAble for (A,) {
    type Output<'world> = (A::Output<'world>,);
    fn get<'world>(world: &'world Treecs, entity: EntityKey) -> Option<Self::Output<'world>> {
        A::get(world, entity).map(|x| (x,))
    }
}
impl<A: QueryAble, B: QueryAble> QueryAble for (A, B) {
    type Output<'world> = (A::Output<'world>, B::Output<'world>);
    fn get<'world>(world: &'world Treecs, entity: EntityKey) -> Option<Self::Output<'world>> {
        let a = A::get(world, entity)?;
        let b = B::get(world, entity)?;
        Some((a, b))
    }
}

pub struct Query<
    'world,
    Q: QueryAble,
    I: QueryIter<'world, Q> = crate::treecs::iterators::breadth::BreadthIter<'world>,
> {
    iter: I,
    _ph: PhantomData<&'world Q>,
}
impl<'world, Q: QueryAble, I: WorldIter<'world, Q>> Query<'world, Q, I> {
    pub fn new(world: &'world Treecs) -> Self {
        Self {
            _ph: PhantomData,
            iter: I::new(&world),
        }
    }
}
impl<'world, Q: QueryAble> Query<'world, Q, ParentIter<'world>> {
    pub fn new_parent(world: &'world Treecs, key: EntityKey) -> Self {
        Self {
            _ph: PhantomData,
            iter: ParentIter::new(world, key),
        }
    }
}
impl<'world, Q: QueryAble, I: QueryIter<'world, Q>> Iterator for Query<'world, Q, I> {
    type Item = <I::Info as InfoTransform<'world, Q>>::Output;
    //type Item = Q::Output<'world>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let key = self.iter.next()?;
            let x = self.iter.transform(key.get_key());
            if let Some(x) = x {
                return Some(key.transform(x));
            }
        }
    }
}
impl<'world, Q: QueryAble, I: WorldIter<'world, Q>> Query<'world, Q, I>{
    pub fn restart(self) -> Self {
        Self {
            iter: self.iter.restart(),
            _ph: PhantomData
        }
    }
}
impl<'world, Q: QueryAble> Query<'world, Q, BreadthIter<'world, Forward>>{
    pub fn reverse(self) -> Query<'world, Q, BreadthIter<'world, Reversed>>{
        Query {
            iter: self.iter.reverse::<Q>(),
            _ph: PhantomData
        }
    }
}
impl<'world, Q: QueryAble> Query<'world, Q, BreadthIter<'world, Reversed>> {
    pub fn reverse(self) -> Query<'world, Q, BreadthIter<'world, Forward>> {
        Query {
            iter: self.iter.reverse::<Q>(),
            _ph: PhantomData
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::treecs::iterators::breadth::BreadthInfo;
    use crate::treecs::iterators::breadth::BreadthIter;
    use crate::treecs::iterators::parent::ParentIter;
    use crate::treecs::test_utils::*;
    use crate::treecs::Treecs;

    use super::Query;

    #[test]
    fn query_breadth_first() {
        let mut world = Treecs::new();
        let entity1 = world.add(world.root()).unwrap();
        let entity2 = world.add(world.root()).unwrap();

        world.register(entity1, Position::new(0, 0));
        world.register(entity2, Position::new(1, 1));

        let entity1_1 = world.add(entity1).unwrap();
        world.register(entity1_1, Position::new(2, 2));

        let mut query: Query<&Position, BreadthIter> = Query::new(&world);
        // root doesnt have a component so is ignored
        macro_rules! assert_eqx {
            ($l:expr, Some(($a:expr, $b:expr))) => {
                let a = $l;
                assert_eq!(a.as_ref().map(|x| &x.0), Some(&$a));
                assert_eq!(a.as_ref().map(|x| &x.1).map(|x| &**x), Some($b), "{:?}: {:?}", $a, $b)
            };
            ($l:expr, None) => {
                let a = $l;
                assert_eq!(a.as_ref().map(|x| &x.0), None);
                assert_eq!(a.as_ref().map(|x| &x.1).map(|x| &**x).as_deref(), None)
            }
        }
        /*for (a,b) in query {
            println!("{a:?} {:?}", b.deref());
        }
        panic!();*/
        assert_eqx!(query.next(), Some((BreadthInfo::Other ,&Position::new(0, 0))));
        assert_eqx!(query.next(), Some((BreadthInfo::Other, &Position::new(2, 2))));
        assert_eqx!(query.next(), Some((BreadthInfo::MoveUp, &Position::new(2, 2))));
        assert_eqx!(query.next(), Some((BreadthInfo::MoveUp, &Position::new(0, 0))));
        assert_eqx!(query.next(), Some((BreadthInfo::Other, &Position::new(1, 1))));
        assert_eqx!(query.next(), Some((BreadthInfo::MoveUp, &Position::new(1, 1))));
        assert_eqx!(query.next(), None);
    }
    #[test]
    fn query_parent() {
        let mut world = Treecs::new();
        let entity_1 = world.add(world.root()).unwrap();
        let entity_1_1 = world.add(entity_1).unwrap();
        let entity_1_1_1 = world.add(entity_1_1).unwrap();

        let entity_2 = world.add(world.root()).unwrap();
        let entity_1_2 = world.add(entity_1).unwrap();

        world.register(entity_1, Position::new(0, 0));
        world.register(entity_2, Position::new(1, 1));
        world.register(entity_1_1_1, Position::new(2, 2));

        let mut query: Query<&Position, ParentIter> = Query::new_parent(&world, entity_1_1_1);
        assert_eq!(query.next().as_deref(), Some(&Position::new(2, 2)));
        assert_eq!(query.next().as_deref(), Some(&Position::new(0, 0)));
        assert_eq!(query.next().as_deref(), None);
    }
    #[test]
    fn query_multiple() {
        let mut world = Treecs::new();
        let entity_1 = world.add(world.root()).unwrap();
        let entity_2 = world.add(world.root()).unwrap();
        let entity_3 = world.add(world.root()).unwrap();

        world.register(entity_1, Position::new(1, 1));
        world.register(entity_1, Name::new("entity1"));

        world.register(entity_2, Position::new(2, 2));

        world.register(entity_3, Position::new(3, 3));
        world.register(entity_3, Name::new("entity3"));

        let mut query: Query<(&Position, &Name)> = Query::new(&world);
        macro_rules! assert_eqx {
            ($l:expr, Some(($a:expr, ($b:expr, $c:expr)))) => {
                let a = $l;
                let a = a.as_ref();
                assert_eq!(a.map(|x| &x.0), Some(&$a));
                assert_eq!(a.map(|x| &x.1.0).map(|x| &**x), Some($b));
                assert_eq!(a.map(|x| &x.1.1).map(|x| &**x), Some($c))
            };
            ($l:expr, None) => {
                let a = $l;
                let a = a.as_ref();
                assert_eq!(a.map(|x| &x.0), None);
                assert_eq!(a.map(|x| &x.1.0).map(|x| &**x), None);
                assert_eq!(a.map(|x| &x.1.1).map(|x| &**x), None)
            }
        }
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::Other, (&Position::new(1, 1), &Name::new("entity1"))))
        );
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::MoveUp, (&Position::new(1,1), &Name::new("entity1"))))
        );
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::Other ,(&Position::new(3, 3), &Name::new("entity3"))))
        );
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::MoveUp ,(&Position::new(3, 3), &Name::new("entity3"))))
        );
        assert_eqx!(query.next(), None);
    }
    #[test]
    fn query_multiple_with_optional() {
        let mut world = Treecs::new();
        let entity_1 = world.add(world.root()).unwrap();
        let entity_2 = world.add(world.root()).unwrap();
        let entity_3 = world.add(world.root()).unwrap();

        world.register(entity_1, Position::new(1, 1));
        world.register(entity_1, Name::new("entity1"));

        world.register(entity_2, Position::new(2, 2));

        world.register(entity_3, Position::new(3, 3));
        world.register(entity_3, Name::new("entity3"));

        let mut query: Query<(&Position, Option<&Name>)> = Query::new(&world);
        macro_rules! assert_eqx {
            ($l:expr, Some(($a:expr, ($b:expr, $c:expr)))) => {
                let a = $l;
                let a = a.as_ref();
                assert_eq!(a.map(|x| &x.0), Some(&$a));
                assert_eq!(a.map(|x| &x.1.0).map(|x| &**x), Some($b));
                assert_eq!(a.map(|x| &x.1.1).map(|x| x.as_ref().map(|x| x.deref())), Some($c))
            };
            ($l:expr, None) => {
                assert_eq!(($l).map(|x| x.0), None);
                assert_eq!(($l).map(|x| x.1.0).as_deref(), None);
                assert_eq!(($l).map(|x| x.1.1).map(|x| x.map(|y| y.name)), None)
            }
        }
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::Other, (&Position::new(1, 1), Some(&Name::new("entity1")))))
        );
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::MoveUp, (&Position::new(1, 1), Some(&Name::new("entity1")))))
        );
        assert_eqx!(query.next(), Some((BreadthInfo::Other, (&Position::new(2, 2), None))));
        assert_eqx!(query.next(), Some((BreadthInfo::MoveUp, (&Position::new(2, 2), None))));
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::Other, (&Position::new(3, 3), Some(&Name::new("entity3")))))
        );
        assert_eqx!(
            query.next(),
            Some((BreadthInfo::MoveUp, (&Position::new(3, 3), Some(&Name::new("entity3")))))
        );
        assert_eqx!(query.next(), None);
    }
}
