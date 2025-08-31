use std::marker::PhantomData;

use crate::{BuildContext, Component, widget::{Render, Widget}};



pub struct State<'gui, W: ?Sized>{
    _ph: PhantomData<W>,
    build_ctx: BuildContext<'gui>
}

impl<'gui, W> State<'gui, W>
where 
    W: Component
{
    pub fn new(build_ctx: BuildContext<'gui>) -> Self {
        Self{
            _ph: PhantomData,
            build_ctx
        }
    }
    pub fn update<F, R>(&self, f: F) -> StateTransaction
    where W: Widget<R>, F: Fn(&W, &mut <W as Widget<R>>::State) + 'static  {
        let a = move |ctx: &mut BuildContext| {
            let state = ctx.get_component_mut::<<W as Widget<R>>::State>().expect("every element always has a state");
            let data = ctx.get_component::<W>().expect("every element always has a widget");
            (f)(data, state)
        };
        StateTransaction{
            func: Box::new(a)
        }
    }
    pub fn read<R>(&self) -> &<W as Widget<R>>::State
    where W: Widget<R>
    {
        self.build_ctx.get_component::<<W as Widget<R>>::State>().unwrap()
    }
}

pub struct StateTransaction{
    func: Box<dyn Fn(&mut BuildContext)>
}