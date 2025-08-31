use std::marker::PhantomData;

use crate::{BuildContext, WidgetContext, renderer::Renderer, state::State, treecs::component::{Component, ComponentEntry, ComponentEntryMut}};







pub trait RenderAble<R> {
    fn mount<'gui>(&self, context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R>;
}


pub trait Widget<R>: RenderAble<R> {
    type State: Component;

    fn create_state(&self) -> Self::State;

    fn build<'gui>(&self, state: State<Self>, context: &mut BuildContext<'gui>) -> impl RenderAble<R> + 'static;
}

/*impl<R, T> RenderAble<R> for T
where T: Widget<R>,
    R: Renderer
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        context.insert_component(self.create_state());
        let mut build_context = context.get_buildcontext();
        let build = {
            let state = context.get_component::<<Self as Widget<R>>::State>().unwrap();
            <Self as Widget<R>>::build(&self, state, &mut build_context)
        };
        build.mount(context)
    }
}
*/


pub trait Render<Widget: ?Sized>: Sized + 'static
{
    // add a change or update method that allows a user to change data in the renderer as optimisation?
    fn mount<'gui>(widget: &Widget, context: &mut WidgetContext<'gui, Self>);
    fn after_mount<'gui>(widget: &Widget, context: &mut WidgetContext<'gui, Self>){}
}

