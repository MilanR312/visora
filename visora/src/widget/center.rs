use std::marker::PhantomData;

use visora_core::{renderer::Renderer, widget::{Render, RenderAble, Widget}, WidgetContext};
use visora_macros::RenderAble;


pub struct Center<W>{
    child: W
}
impl<W:'static> Center<W>
{
    pub fn new(data: W) -> Self {
        Self {
            child: data
        }
    }
}
impl<R, W> RenderAble<R> for Center<W>
where W: Widget<R>,
      R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        let child = context.new_child();
        self.child.mount(child).to_parent().unwrap()
    }
}


/*impl<R> Widget<R> for Center<R>
where R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        let child = context.new_child();
        self.child.mount(child).to_parent().unwrap()
    }
}
*/

