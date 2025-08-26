use std::ops::DerefMut;

use visora::widget::{
    button::TextButton, center::Center, container::{Container, EdgeInsets}, list::Hlist, text::{RichText, Text, Vlist}
};
use visora_core::{
    color::Color, renderer::Renderer, treecs::query::Query, widget::{State, StatefulWidget, StatelessWidget, Widget}, Gui, WidgetContext
};
use visora_macros::{StatefulWidget, StatelessWidget};
use visora_ssr::html::HtmlRenderer;

#[derive(StatelessWidget)]
struct Name;
impl<R: visora_ssr::SupportedWidgets> StatelessWidget<R> for Name {
    fn build<'gui>(
        &self,
        context: &mut visora_core::BuildContext<'gui>,
    ) -> impl Widget<R> + 'static {
        Text::new("john doe")
    }
}


#[derive(StatelessWidget)]
struct Attribution{
    name: String
}
impl<R> StatelessWidget<R> for Attribution
where
    R: visora_ssr::SupportedWidgets,
{
    fn build<'gui>(
        &self,
        context: &mut visora_core::BuildContext<'gui>,
    ) -> impl Widget<R> + 'static {
        Center::new(
            Container::new()
                .with_child(
                    Vlist::new()
                        .add(Text::new("this is a working demo of modulars"))
                        .add(Hlist::new().add(Text::new("made by:")).add(Text::new(&self.name)))
                        .add(Text::lorem(50))
                        .add(
                            RichText::new("and in color :)".to_owned())
                                .with_color(Color::new_hex(0x0000ff)),
                        ),
                )
                .with_insets(EdgeInsets::all(20)),
        )
    }
}


#[derive(StatefulWidget)]
struct Counter {
    start: u64,
    end: u64
}

struct CounterState(u64);
impl State for CounterState{}

impl<R: visora_ssr::SupportedWidgets> StatefulWidget<R> for Counter {
    type State = CounterState;
    fn create_state(&self) -> Self::State {
        CounterState(self.start)
    }
    fn build<'gui>(&self, state: &Self::State, context: &mut visora_core::BuildContext<'gui>) -> impl Widget<R> + 'static {
        Vlist::new()
            .add(
                RichText::new(format!("Count is {}", state.0))
                .with_color(Color::new_hex(0x000000ff))
            )
            .add(
                TextButton::new(
                        Text::new("Press me")
                    )
                    .on_click(context, |state: &mut Self::State| {
                        state.changed();
                    })
            )
        
    }
}
// question:
// what to do when a method has an on_click? 
// Should State be owned and cheaply clonable?

fn main() {
    use visora_core::widget::{StatelessWidget, Widget};
    let mut gui = Gui::new(HtmlRenderer);

    let widget = Counter{start: 1, end: 5};
    let context = gui.root_widget_context();
    widget.mount(context);

    gui.render();
}
