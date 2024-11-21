pub mod treecs;
pub mod widget;
pub mod color;
pub mod renderer;
use std::marker::PhantomData;

use renderer::Renderer;
pub use treecs::component::Component;
use treecs::{component::{ComponentEntry, ComponentEntryMut, ComponentStore}, entity::Entity, iterators::{breadth::BreadthIter, QueryIter}, query::{Query, QueryAble}, EntityKey, Treecs};

pub struct Gui<R> {
    tree: Treecs,
    renderer: R
}
impl<R: Renderer> Gui<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            tree: Treecs::new(),
            renderer
        }
    }
    pub fn root_widget_context(&mut self) -> WidgetContext<'_, R> {
        let key = self.tree.root();
        WidgetContext {
            tree: &mut self.tree,
            key,
            _ph: PhantomData,
        }
    }
    pub fn root_build_context(&mut self) -> BuildContext<'_/*, R*/> {
        let key = self.tree.root();
        BuildContext {
            store: self.tree.store(),
            key,
            //_ph: PhantomData,
        }
    }
    pub fn tree(&self) -> &Treecs{
        &self.tree
    }
    pub fn renderer(&mut self) -> &mut R {
        &mut self.renderer
    }
    pub fn render(&mut self){
        let query = Query::new(&self.tree);
        self.renderer.render(query);
    }
}

pub struct BuildContext<'gui/*, R*/> {
    store: &'gui ComponentStore,
    key: EntityKey,
    //_ph: PhantomData<R>,
}
impl<'gui> BuildContext<'gui> {
    pub fn get_component<Q: Component>(&self) -> Option<&Q> {
        self.store.get_component(self.key)
    }
    pub fn get_component_mut<Q: Component>(&self) -> Option<&mut Q>{
        self.store.get_component_mut(self.key)
    }
    pub fn insert_component<Q: Component>(&self, comp: Q) {
        self.store.add_component(self.key, comp);
    }
    
}
pub struct WidgetContext<'gui, R> {
    tree: &'gui mut Treecs,
    key: EntityKey,
    _ph: PhantomData<R>,
}

impl<'gui, R: Renderer> WidgetContext<'gui, R> {
    pub fn insert_component<Q: Component>(&mut self, comp: Q){
        self.tree.register(self.key, comp);
    }
    pub fn mount_renderer(&mut self, renderer: R::RenderItem){
        self.tree.register(self.key, renderer);        
    }
    pub fn get_renderer(&mut self) -> Option<&mut R::RenderItem>{
        self.get_component_mut::<R::RenderItem>()
    }
    pub fn get_component<Q: Component>(&self) -> Option<&Q> {
        self.tree.get_component(self.key)
    }
    pub fn get_component_mut<Q: Component>(&self) -> Option<&mut Q>{
        (&*self.tree).get_component_mut(self.key)
    }
    pub fn new_child(self) -> Self {
        let child = self.tree.add(self.key).unwrap();
        Self {
            tree: self.tree,
            key: child,
            _ph: PhantomData
        }
    }
    pub fn to_parent(self) -> Option<Self> {
        let linkdata = self.tree.linkdata(self.key).unwrap();
        let parent = *linkdata.parent().as_ref()?;
        Some(Self {
            tree: self.tree,
            key: parent,
            _ph: PhantomData
        })
    }
    pub fn get_buildcontext(&self) -> BuildContext<'_> {
        BuildContext {
            key: self.key,
            store: self.tree.store()
        }
    }
}

//impl<'gui, R: Component> BuildContext<'gui, R> {}

