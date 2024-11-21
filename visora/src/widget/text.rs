use lipsum::lipsum;
use visora_core::{
    color::Color, renderer::Renderer, widget::{Render, StatelessWidget, Widget}, BuildContext, Component, WidgetContext
};

pub struct Text {
    pub data: String,
    pub is_bold: bool
}
impl Text {
    pub fn new(data: &str) -> Self {
        Self {
            data: data.to_owned(),
            is_bold: false
        }
    }
    pub fn lorem(len: usize) -> Self {
        Self {
            data: lipsum(len),
            is_bold: false
        }
    }
    pub fn set_bold(mut self) -> Self {
        self.is_bold = true;
        self
    }
}
impl<R> Widget<R> for Text 
where R: Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(&self, &mut context);
        context
    }
}


pub enum Align{
    /// aligns text on the left edge
    Left,
    /// aligns text on the right edge
    Right,
    /// alligns text in the center
    Center,
    /// stretches the lines so that each line has equal width
    Justify
}
pub enum Direction {
    LeftToRight,
    RightToLeft
}
pub struct RichText{
    text: String,
    alignment: Align,
    direction: Direction,
    color: Color
}
impl Default for RichText {
    fn default() -> Self {
        Self {
            text: String::new(),
            alignment: Align::Left,
            direction: Direction::LeftToRight,
            color: Color::new_hex(0xffffff)
        }
    }
}
impl RichText {
    pub fn lorem(len: usize) -> Self {
        let text = lipsum(len);
        Self {text, ..Default::default()}
    }
    pub fn new(text: String) -> Self {
        Self {text, ..Default::default()}
    }
    pub fn with_align(mut self, align: Align) -> Self {
        self.alignment = align;
        self
    }
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn alignment(&self) -> &Align {
        &self.alignment
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    pub fn color(&self) -> &Color {
        &self.color
    }
}
impl<R: Render<Self>> Widget<R> for RichText {
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(&self, &mut context);
        context
    }
}


pub struct Vlist<R> {
    pub data: Vec<Box<dyn Widget<R>>>,
}
impl<R: Renderer> Vlist<R> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn add(mut self, x: impl Widget<R> + 'static) -> Self {
        self.data.push(Box::new(x));
        self
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl<R> Widget<R> for Vlist<R>
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

