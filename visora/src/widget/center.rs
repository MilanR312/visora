use std::marker::PhantomData;

use visora_core::{renderer::Renderer, widget::{Render, Widget}, WidgetContext};



pub struct Center<R>{
    child: Box<dyn Widget<R>>
}
impl<R: Renderer> Center<R> {
    pub fn new(data: impl Widget<R> + 'static) -> Self {
        Self {
            child: Box::new(data)
        }
    }
}
impl<R> Widget<R> for Center<R>
where R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        let child = context.new_child();
        self.child.mount(child).to_parent().unwrap()
    }
}


