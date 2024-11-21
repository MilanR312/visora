use visora_core::{color::Color, renderer::Renderer, widget::{Render, Widget}, WidgetContext};

#[derive(Debug, Clone, Copy)]
pub struct EdgeInsets{
    left: u8,
    top: u8,
    right: u8,
    bottom: u8
}
impl EdgeInsets {
    pub fn all(value: u8) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value
        }
    }
}
impl EdgeInsets {
    pub fn left(&self) -> u8 {
        self.left
    }
    pub fn right(&self) -> u8 {
        self.right
    }
    pub fn top(&self) -> u8 {
        self.top
    }
    pub fn bottom(&self) -> u8 {
        self.bottom
    }
    pub fn is_even(&self) -> bool {
        self.top == self.bottom && self.top == self.left && self.top == self.right
    }
}


pub struct Container<R>{
    child: Option<Box<dyn Widget<R>>>,
    insets: EdgeInsets,
    bg: Color
}
impl<R: Renderer> Container<R>{
    pub fn new() -> Self {
        Self {
            child: None,
            insets: EdgeInsets::all(0),
            bg: Color::new_argb(255, 255, 255, 255)
        }
    }
    pub fn with_child(mut self, x: impl Widget<R> + 'static) -> Self {
        self.child = Some(Box::new(x));
        self
    }
    pub fn with_insets(mut self, insets: EdgeInsets) -> Self {
        self.insets = insets;
        self
    }
    pub fn insets(&self) -> &EdgeInsets {
        &self.insets
    }
    pub fn bg(&self) -> &Color {
        &self.bg
    }
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }
}

impl<R> Widget<R> for Container<R>
where R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        match &self.child {
            Some(x) => {
                let child = context.new_child();
                x.mount(child).to_parent().unwrap()
            },
            None => context
        }
    }
}