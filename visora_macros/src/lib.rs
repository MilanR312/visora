use proc_macro::{TokenStream};
use syn::{parse_macro_input, DataStruct, DeriveInput, Ident, Visibility};


#[proc_macro_derive(StatelessWidget)]
pub fn stateless_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    quote::quote!{
        impl<R: visora_core::renderer::Renderer> visora_core::widget::Widget<R> for #name
        where
            Self: visora_core::widget::StatelessWidget<R>,
        {
            fn mount<'gui>(&self, context: visora_core::WidgetContext<'gui, R>) -> visora_core::WidgetContext<'gui, R> {
                let mut bcontext = context.get_buildcontext();
                let a = <Self as visora_core::widget::StatelessWidget<R>>::build(&self, &mut bcontext);
                a.mount(context)
            }
        }
    }.into()
}
#[proc_macro_derive(StatefulWidget)]
pub fn stateful_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    quote::quote! {
        impl<R: Renderer> Widget<R> for #name
        where Self: StatefulWidget<R>
        {
            fn mount<'gui>(&self, mut context: visora_core::WidgetContext<'gui, R>) -> visora_core::WidgetContext<'gui, R> {
                context.insert_component(<Self as StatefulWidget<R>>::create_state(&self));
                let mut bcontext = context.get_buildcontext();
                let a = {
                    let mut state = context.get_component_mut::<<Self as StatefulWidget<R>>::State>().unwrap();
                    <Self as StatefulWidget<R>>::build(&self, state.deref_mut(), &mut bcontext)
                };
                a.mount(context)
            }
        }
    }.into()
}
