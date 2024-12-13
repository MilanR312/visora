use component::{Component, ComponentEntry, ComponentEntryMut, ComponentStore};
use entity::Entity;
use linkdata::LinkData;
use slotmap::{new_key_type, SlotMap};

pub mod children;
pub mod component;
pub mod entity;
pub mod iterators;
pub mod linkdata;
pub mod query;

#[cfg(test)]
pub mod test_utils;

new_key_type! {
    pub struct EntityKey;
}

// TODO: change slotmap for slab
pub struct Treecs {
    linkdata: SlotMap<EntityKey, LinkData>,
    root: EntityKey,
    components: ComponentStore,
}
impl Treecs {
    pub fn new() -> Self {
        let mut linkdata = SlotMap::with_key();
        let root = linkdata.insert(LinkData::new_empty());
        Self {
            linkdata,
            root,
            components: ComponentStore::new(),
        }
    }
    pub fn store(&self) -> &ComponentStore {
        &self.components
    }
    pub fn linkdata(&self, key: EntityKey) -> Option<&LinkData> {
        self.linkdata.get(key)
    }
    pub fn linkdata_mut(&mut self, key: EntityKey) -> Option<&mut LinkData> {
        self.linkdata.get_mut(key)
    }
    pub fn root(&self) -> EntityKey {
        self.root
    }

    pub fn contains(&self, entity: EntityKey) -> bool {
        self.linkdata.contains_key(entity)
    }
    pub fn entity_count(&self) -> usize {
        self.linkdata.len()
    }

    pub fn add(&mut self, parent: EntityKey) -> Option<EntityKey> {
        if !self.contains(parent) {
            return None;
        }
        let key = self.linkdata.insert(LinkData::new_with_parent(parent));
        self.linkdata_mut(parent)
            .unwrap()
            .children_mut()
            .push_right(key);
        Some(key)
    }
    pub fn remove(&mut self, entity: EntityKey) -> Option<()> {
        if !self.contains(entity) {
            return None;
        }
        let entity_linkdata = self.linkdata(entity).unwrap();
        match entity_linkdata.parent() {
            // if the entity has a parent remove the entity from the children
            Some(x) => {
                self.linkdata_mut(*x).unwrap().children_mut().remove(entity);
            }
            // if the element has no parent it means we removed the root
            None => {
                self.linkdata.clear();
                let new_root = self.linkdata.insert(LinkData::new_empty());
                self.root = new_root;
            }
        }
        // remove all children
        let mut to_remove_stack = vec![entity];
        while let Some(x) = to_remove_stack.pop() {
            let linkdata = self.linkdata.remove(x).unwrap();
            self.components.remove_entity(x);

            for child in linkdata.children() {
                to_remove_stack.push(*child);
            }
        }

        Some(())
    }
}

//component related impls
impl Treecs {
    pub fn register<T: Component>(&mut self, entity: EntityKey, component: T) {
        self.components.add_component(entity, component)
    }
    pub fn get_component<T: Component>(&self, entity: EntityKey) -> Option<&T> {
        self.components.get_component(entity)
    }
    pub fn get_component_mut<T:Component>(&self, entity: EntityKey) -> Option<&mut T>{
        self.components.get_component_mut(entity)
    }
    pub fn remove_component<T: Component>(&mut self, entity: EntityKey) -> Option<T> {
        self.components.remove_component(entity)
    }
}

// entity related impls
/*impl World {
    pub fn entity<'world>(&'world self, key: EntityKey) -> Option<Entity<'world>>{
        Entity::new(self, key)
    }
}*/

/// tests in this module are intentionally small and the tree is tested more in the Node file
#[cfg(test)]
mod tests {
    use crate::treecs::{linkdata::LinkData, Treecs};

    #[test]
    fn add_child() {
        let mut world = Treecs::new();

        let entity1 = world.add(world.root()).unwrap();

        assert_eq!(world.linkdata.len(), 2);
        assert_eq!(
            world.linkdata(entity1),
            Some(&LinkData::new_with_parent(world.root()))
        );
        let mut iter = world.linkdata(world.root()).unwrap().children().iter();
        assert_eq!(iter.next(), Some(&entity1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn remove_child() {
        let mut world = Treecs::new();

        let entity1 = world.add(world.root()).unwrap();
        let entity2 = world.add(world.root()).unwrap();
        assert_eq!(world.entity_count(), 3); // includes the root node

        world.remove(entity1);
        assert_eq!(world.entity_count(), 2);

        assert_eq!(
            world.linkdata(entity2),
            Some(&LinkData::new_with_parent(world.root()))
        );
        let mut iter = world.linkdata(world.root()).unwrap().children().iter();
        assert_eq!(iter.next(), Some(&entity2));
        assert_eq!(iter.next(), None);
    }
    use crate::treecs::test_utils::*;

    #[test]
    fn get_component() {
        let mut world = Treecs::new();
        let entity1 = world.add(world.root()).unwrap();
        let entity2 = world.add(world.root()).unwrap();

        let entity1_1 = world.add(entity1).unwrap();

        world.register(entity1, Position::new(0, 0));
        world.register(entity1_1, Position::new(1, 1));
        world.register(entity1, Name::new("e1"));
        world.register(entity2, Name::new("e2"));
        assert_eq!(
            world.get_component::<Position>(entity1).as_deref(),
            Some(&Position::new(0, 0))
        );
        assert_eq!(
            world.get_component::<Position>(entity1_1).as_deref(),
            Some(&Position::new(1, 1))
        );
        assert_eq!(world.get_component::<Position>(entity2).as_deref(), None);

        assert_eq!(world.get_component::<Name>(entity1).as_deref(), Some(&Name::new("e1")));
        assert_eq!(world.get_component::<Name>(entity2).as_deref(), Some(&Name::new("e2")));
        assert_eq!(world.get_component::<Name>(entity1_1).as_deref(), None);
    }

    #[test]
    fn remove_component() {
        let mut world = Treecs::new();
        let entity = world.add(world.root()).unwrap();

        world.register(entity, Position::new(1, 1));
        world.register(entity, Name::new("entity"));

        assert_eq!(
            world.get_component::<Position>(entity).as_deref(),
            Some(&Position::new(1, 1))
        );
        assert_eq!(
            world.get_component::<Name>(entity).as_deref(),
            Some(&Name::new("entity"))
        );

        let name = world.remove_component::<Name>(entity);
        assert_eq!(name, Some(Name::new("entity")));

        assert_eq!(
            world.get_component::<Position>(entity).as_deref(),
            Some(&Position::new(1, 1))
        );
        assert_eq!(world.get_component::<Name>(entity).as_deref(), None);
    }
}
