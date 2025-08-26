use std::marker::PhantomData;

use crate::{treecs::component::{Component, ComponentEntry, ComponentEntryMut}, BuildContext, WidgetContext};



mod test {

    trait Widget{
        fn mount(&self){}
    }
    trait RenderWidget: Widget{
        fn attach(&self){}
    }
    trait StatelessWidget: Widget{
        fn build(&self) -> impl Widget;
    }

    struct Text;
    impl Widget for Text{
        fn mount(&self) {
            self.attach();
        }
    }
    impl RenderWidget for Text {
        fn attach(&self) {
            println!("attached Text");
        }
    }

    struct Vlist(Vec<Box<dyn Widget>>);
    impl Widget for Vlist {
        fn mount(&self) {
            self.attach();
        }
    }
    impl RenderWidget for Vlist {
        fn attach(&self) {
            println!("attached Vlist");
            for x in &self.0 {
                x.mount();
            }
        }
    }

    struct Attribution;
    impl Widget for Attribution {
        fn mount(&self) {
            let a = self.build();
            println!("built attribution");
            a.mount();
        }
    }
    impl StatelessWidget for Attribution {
        fn build(&self) -> impl Widget {
            Vlist(
                vec![
                    Box::new(Text),
                    Box::new(Text),
                ]
            )
        }
    }


}


pub trait Widget<R>{
    fn mount<'gui>(&self, context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R>;
}
pub trait StatelessWidget<R>: Widget<R> {
    fn build<'gui>(&self, context: &mut BuildContext<'gui>) -> impl Widget<R> + 'static;
}

pub trait State{
    fn changed(&self){}
}
pub trait StatefulWidget<R>: Widget<R>  {
    type State: Component + State;
    fn get_state_mut<'gui, 'ctx>(context: &'ctx mut BuildContext<'gui>) -> &'ctx mut Self::State {
        context.get_component_mut::<Self::State>().expect("state not found, did you forget to add it in the add_state method?")
    }
    fn create_state(&self) -> Self::State;
    fn build<'gui>(&self, state: &Self::State, context: &mut BuildContext<'gui>) -> impl Widget<R> + 'static;
}






pub trait Render<Widget: ?Sized>: Sized + 'static
{
    // add a change or update method that allows a user to change data in the renderer as optimisation?
    fn mount<'gui>(widget: &Widget, context: &mut WidgetContext<'gui, Self>);
    fn after_mount<'gui>(widget: &Widget, context: &mut WidgetContext<'gui, Self>){}
}

