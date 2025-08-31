use std::marker::PhantomData;

use visora_core::{BuildContext, Component, WidgetContext, renderer::Renderer, state::StateTransaction, widget::{Render, RenderAble, Widget}};

use super::text::Text;

struct CallBack<State>(Box<dyn Fn(&mut State) + 'static + Send + Sync>);



pub struct TextButton{
    child: Text,
    update: Option<StateTransaction>
}
impl TextButton
{
    pub fn new(text: Text) -> Self {
        Self{
            child: text,
            update: None
        }
    }
    pub fn on_click(mut self, update: StateTransaction) -> Self
    {
        self.update = Some(update);
        self
    }
    /*pub fn handle_click<'gui, State: Component>(&self, context: &mut BuildContext<'gui>){
        let Some(f) = context.get_component::<fn(&mut State)>() else { return };
        let Some(state) = context.get_component_mut::<State>() else { return };
        f(state);
    }*/
    // how to implement the call(), decay to fn pointer maybe?
}

impl<R> RenderAble<R> for TextButton
where R: Renderer + Render<Self> + Render<Text>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        let child = context.new_child();
        self.child.mount(child).to_parent().unwrap()
    }
}

