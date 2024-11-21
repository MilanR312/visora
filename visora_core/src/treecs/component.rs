use std::{any::TypeId, collections::HashMap, ops::{Deref, DerefMut}, sync::Arc};

use dashmap::{mapref::one::{MappedRef, MappedRefMut}, DashMap};

use crate::treecs::EntityKey;

pub trait Component: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Component for T {}

pub struct ComponentStore{
    components: DashMap<TypeId, HashMap<EntityKey, Box<dyn Component>>>,
    //components: HashMap<TypeId, Arc<DashMap<EntityKey, Box<dyn Component>>>>,
}
pub struct ComponentEntryMut<'a, T>(MappedRefMut<'a, EntityKey, Box<dyn Component>, T>);
impl<'a, T> Deref for ComponentEntryMut<'a, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.value()
    }
}
impl<'a, T> DerefMut for ComponentEntryMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.value_mut()
    }
}
pub struct ComponentEntry<'a, T>(MappedRef<'a, EntityKey, Box<dyn Component>, T>);
impl<'a, T> Deref for ComponentEntry<'a, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.value()
    }
}


impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: DashMap::new(),
        }
    }
    pub fn add_component<T: Component>(&self, entity: EntityKey, component: T) {
        let mut map = self.components.entry(TypeId::of::<T>()).or_default();
        map.insert(entity, Box::new(component));
    }
    /// removes all mentions of an entity in the componentstore
    pub fn remove_entity(&self, entity: EntityKey) {
        for mut map in self.components.iter_mut(){
            map.remove(&entity);
        }
    }

    /*pub fn get_component<T: Component>(&self, entity: EntityKey) -> Option<&T> {
        let map = self.components.get(&TypeId::of::<T>())?;
        let val = map.get(&entity)?;
        let val = &**val as *const dyn Component as *const T;
        let val = unsafe { &*val };
        Some(val)
    }*/
    /*pub fn get_component<T: Component>(&self, entity: EntityKey) -> Option<ComponentEntry<'_, T>>{
        let map = self.components.get(&TypeId::of::<T>())?;
        let val = map.get(&entity)?;
        let out = {
            let val = &**val as *const dyn Component as *const T;
            unsafe { &*val }
        };
        Some(ComponentEntry(out))
    }
    pub fn get_component_mut<T: Component>(&self, entity: EntityKey) -> Option<ComponentEntryMut<'_, T>>{
        let map = self.components.get(&TypeId::of::<T>())?;
        let val = map.get_mut(&entity)?;
        let out = val.map(|x| {
            let x = &mut **x as *mut dyn Component as *mut T;
            unsafe { &mut *x }
        });
        Some(ComponentEntryMut(out))
    }*/
    pub fn get_component<T: Component>(&self, entity: EntityKey) -> Option<&T>{
        let map = self.components.get(&TypeId::of::<T>())?;
        let val = map.get(&entity)?;
        let out = {
            let val = &**val as *const dyn Component as *const T;
            unsafe { &*val }
        };
        Some(out)
    }
    pub fn get_component_mut<T: Component>(&self, entity: EntityKey) -> Option<&mut T>{
        let mut map = self.components.get_mut(&TypeId::of::<T>())?;
        let val = map.get_mut(&entity)?;
        let out = {
            let x = &mut **val as *mut dyn Component as *mut T;
            unsafe { &mut *x }
        };
        Some(out)
    }
    pub fn remove_component<T: Component>(&mut self, entity: EntityKey) -> Option<T> {
        let mut map = self.components.get_mut(&TypeId::of::<T>())?;
        let val = map.remove(&entity)?;
        let val = Box::leak(val) as *mut dyn Component as *mut T;
        let val = unsafe { Box::from_raw(val) };
        Some(*val)
    }
}
