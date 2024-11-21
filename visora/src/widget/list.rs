use visora_core::{renderer::Renderer, widget::{Render, Widget}, WidgetContext};



pub struct Hlist<R> {
    pub data: Vec<Box<dyn Widget<R>>>,
}
impl<R: Renderer> Hlist<R> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn add(mut self, x: impl Widget<R> + 'static) -> Self {
        self.data.push(Box::new(x));
        self
    }
}
impl<R> Widget<R> for Hlist<R>
where R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        for x in &self.data {
            let child = context.new_child();
            context = x.mount(child).to_parent().unwrap();
        }
        context
    }

}