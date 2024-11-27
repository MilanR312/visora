use std::marker::PhantomData;

use visora_core::{renderer::Renderer, widget::{Render, Widget}, BuildContext, Component, WidgetContext};


pub struct Button2<F>(PhantomData<F>);
impl<F: Component> Button2<F> {
    pub fn new<'gui>(context: &mut BuildContext<'gui>, func: F) -> Self {
        context.insert_component(func);
        Self(PhantomData)
    }
    pub fn handle_click<'gui>(&self, context: &mut BuildContext<'gui>){
        let Some(x) = context.get_component::<F>() else { return };

    }
}


pub struct Button{
    // function isnt stored here, it is stored in the ecs
}
impl Button
{
    pub fn new() -> Self {
        Self{
        }
    }
    pub fn on_click<'gui, State: Component>(self, context: &mut BuildContext<'gui>, func: fn(&mut State)) -> Self
    {
        context.insert_component(func);
        self
    }
    pub fn handle_click<'gui, State: Component>(&self, context: &mut BuildContext<'gui>){
        let Some(f) = context.get_component::<fn(&mut State)>() else { return };
        let Some(state) = context.get_component_mut::<State>() else { return };
        f(state);
    }
    // how to implement the call(), decay to fn pointer maybe?
}

impl<R> Widget<R> for Button
where R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        context
    }
}


