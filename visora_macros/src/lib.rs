use proc_macro::{TokenStream};
use syn::{parse_macro_input, spanned::Spanned, DataStruct, DeriveInput, Ident, PredicateType, Type, TypeParam, Visibility, WherePredicate};


#[proc_macro_derive(RenderAble)]
pub fn renderable(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let ident = Ident::new("R", input.span());
    let params_before = input.generics.params.clone();
    let params_before = if params_before.len() == 0 {
        quote::quote! {}
    } else {
        quote::quote! { < #params_before >}
    };

    {
        let generics = &mut input.generics;
        generics.params.push(syn::GenericParam::Type(TypeParam::from(ident)));
    }

    let wherec = input.generics.make_where_clause();
    let predicate = quote::quote!{R: Renderer}.into();
    let predicate = parse_macro_input!(predicate as WherePredicate);
    wherec.predicates.push(predicate);

    let predicate = quote::quote!{Self: Widget<R>}.into();
    let predicate = parse_macro_input!(predicate as WherePredicate);
    wherec.predicates.push(predicate);


    let a = &input.generics.params;
    let b = &input.generics.where_clause;
    let name = input.ident;
    quote::quote! {
        impl < #a > RenderAble<R> for #name #params_before
        #b
        {
            type StateHandle = ::visora_core::state::State<Self, <Self as Widget<R>>::State>;
            fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
                context.insert_component(self.create_state());
                let mut build_context = context.get_buildcontext();
                /*let build = {
                    let state = context.get_component::<<Self as Widget<R>>::State>().unwrap();
                    <Self as Widget<R>>::build(&self, state, &mut build_context)
                };
                */
                let state = ::visora_core::state::State::new();
                let build = <Self as Widget<R>>::build(&self, state, &mut build_context);

                build.mount(context)
            }
        }
    }.into()
}


/*#[proc_macro_derive(StatelessWidget)]
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
}*/
