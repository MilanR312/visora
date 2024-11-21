use std::marker::PhantomData;

use crate::treecs::{component::Component, linkdata::LinkData, EntityKey, Treecs};

mod sealed {
    pub trait Sealed {}
}

mod mutability {
    use crate::treecs::Treecs;

    use super::sealed::Sealed;

    pub struct Mut<'world>(&'world mut Treecs);
    pub struct Imut<'world>(&'world Treecs);

    pub trait Ref: Sealed {
        fn world(&self) -> &Treecs;
    }
    // every mutable reference can also be used as a regular reference
    pub trait RefMut: Ref {
        fn world_mut(&mut self) -> &mut Treecs;
    }
    impl<'world> Sealed for Imut<'world> {}
    impl<'world> Ref for Imut<'world> {
        fn world(&self) -> &Treecs {
            &self.0
        }
    }
    impl<'world> Sealed for Mut<'world> {}
    impl<'world> Ref for Mut<'world> {
        fn world(&self) -> &Treecs {
            self.0
        }
    }
    impl<'world> RefMut for Mut<'world> {
        fn world_mut(&mut self) -> &mut Treecs {
            self.0
        }
    }

    impl<'world> Imut<'world> {
        pub(crate) fn new(world: &'world Treecs) -> Self {
            Self(world)
        }
    }
    impl<'world> Mut<'world> {
        pub(crate) fn new(world: &'world mut Treecs) -> Self {
            Self(world)
        }
    }
}
pub use mutability::{Imut, Mut, Ref, RefMut};

pub struct Entity<'world, World> {
    world: World,
    key: EntityKey,
    _ph: PhantomData<&'world ()>,
}
impl<'world, World: Ref> Entity<'world, World> {
    fn linkdata(&self) -> &LinkData {
        self.world
            .world()
            .linkdata(self.key)
            .expect("key in entity is always valid")
    }
    /*fn get_component<Q: Component>(&self) -> Option<&Q> {
        self.world.world().get_component(self.key)
    }*/
}
impl<'world, World: RefMut> Entity<'world, World> {
    fn linkdata_mut(&mut self) -> &mut LinkData {
        self.world
            .world_mut()
            .linkdata_mut(self.key)
            .expect("key in entity is always valid")
    }
    fn remove(mut self) {
        self.world.world_mut().remove(self.key);
    }
    pub fn as_ref(&'world self) -> Entity<'world, Imut<'world>> {
        Entity {
            _ph: PhantomData,
            key: self.key,
            world: Imut::new(self.world.world()),
        }
    }
    pub fn add_component<Q: Component>(&mut self, component: Q) {
        self.world.world_mut().register(self.key, component);
    }
    fn remove_component<Q: Component>(&mut self) -> Option<Q> {
        self.world.world_mut().remove_component(self.key)
    }
}

impl<'world, World: Ref> Entity<'world, World> {
    pub fn parent(self) -> Option<Self> {
        let Self { world, key, .. } = self;
        let linkdata = world
            .world()
            .linkdata(key)
            .expect("key in entity is always valid");
        let parent = linkdata.parent().as_ref().cloned()?;
        Some(Self {
            _ph: PhantomData,
            world: world,
            key: parent,
        })
    }
    pub fn child(self, index: usize) -> Option<Self> {
        let Self { world, key, .. } = self;
        let linkdata = world
            .world()
            .linkdata(key)
            .expect("key in entity is always valid");
        let child = linkdata.children().get_child(index).cloned()?;
        Some(Self {
            _ph: PhantomData,
            world: world,
            key: child,
        })
    }
}

impl<'world> Entity<'world, Imut<'world>>{
    pub fn new(ecs: &'world Treecs, key: EntityKey) -> Option<Self>{
        if ecs.contains(key){
            Some(Entity {
                world: Imut::new(ecs),
                key,
                _ph: PhantomData
            })
        } else {
            None
        }
    }
}
impl<'world> Entity<'world, Mut<'world>>{
    pub fn new_mut(ecs: &'world mut Treecs, key: EntityKey) -> Option<Self>{
        if ecs.contains(key){
            Some(Entity {
                world: Mut::new(ecs),
                key,
                _ph: PhantomData
            })
        } else {
            None
        }
    }
}