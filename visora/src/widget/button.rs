use std::marker::PhantomData;

use visora_core::{renderer::Renderer, widget::{Render, Widget}, BuildContext, Component, WidgetContext};

use super::text::Text;

struct CallBack<State>(Box<dyn Fn(&mut State) + 'static + Send + Sync>);



pub struct TextButton{
    child: Text
    // function isnt stored here, it is stored in the ecs
}
// TODO: move to FnMut instead of mut when get_component_mut is more stable
impl TextButton
{
    pub fn new(text: Text) -> Self {
        Self{
            child: text
        }
    }
    pub fn on_click<'gui, State: Component, F>(self, context: &mut BuildContext<'gui>, func: F) -> Self
    where F: Fn(&mut State) + 'static + Send + Sync
    {
        let callback = CallBack(Box::new(func));
        context.insert_component(callback);
        self
    }
    pub fn handle_click<'gui, State: Component>(&self, context: &mut BuildContext<'gui>){
        let Some(callback) = context.get_component::<CallBack<State>>() else { return; };
        let Some(state) = context.get_component_mut::<State>() else { return; };
        (callback.0)(state);
    }
    /*pub fn handle_click<'gui, State: Component>(&self, context: &mut BuildContext<'gui>){
        let Some(f) = context.get_component::<fn(&mut State)>() else { return };
        let Some(state) = context.get_component_mut::<State>() else { return };
        f(state);
    }*/
    // how to implement the call(), decay to fn pointer maybe?
}

impl<R> Widget<R> for TextButton
where R: Renderer + Render<Self> + Render<Text>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        let child = context.new_child();
        self.child.mount(child).to_parent().unwrap()
    }
}


