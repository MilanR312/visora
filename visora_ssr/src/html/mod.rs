use std::{borrow::Cow, collections::HashMap, fmt::Write, fs::File, io::{Read, Write as ioWrite}};

use itertools::Itertools;
use uuid::Uuid;
use visora::widget::{button::TextButton, center::Center, list::Hlist, text::{self, RichText, Text}};
use visora_core::{renderer::Renderer, treecs::iterators::breadth::{BreadthInfo, BreadthIter}, widget::Render};

mod tags;

#[derive(Debug)]
pub struct Attributes{
    on_click: Option<Cow<'static, str>>,
    styles: HashMap<&'static str, Cow<'static, str>>,
    id: String
}
impl Attributes {
    pub fn new() -> Self {
        Self {
            on_click: None,
            styles: HashMap::new(),
            id: Uuid::new_v4().to_string()
        }
    }
    pub fn with_style(mut  self, key: &'static str, value: Cow<'static, str>) -> Self {
        self.add_style(key, value);
        self
    }
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }
    pub fn add_style(&mut self, key: &'static str, value: Cow<'static, str>){
        self.styles.insert(key, value);
    }
    pub fn with_on_click(mut self, method: Cow<'static, str>) -> Self {
        self.add_on_click(method);
        self
    }
    pub fn add_on_click(&mut self, method: Cow<'static, str>){
        self.on_click = Some(method);
    }
    pub fn write(&self, dest: &mut String) -> Result<(), std::fmt::Error>{
        dest.write_char(' ')?;
        if self.styles.len() > 0 {
            dest.write_str("style=\"")?;
            let attrs = self.styles.iter().map(|(key, val)| format!("{key}:{val}")).join(";");
            dest.write_str(&attrs)?;
            dest.write_str("\"")?;
            dest.write_char(' ')?;
        }
        if let Some(click) = &self.on_click {
            dest.write_str("onclick=\"")?;
            dest.write_str(&click)?;
            dest.write_str("\"")?;
        }
        write!(dest, "id=\"{}\"", self.id)?;
        Ok(())
    }
}

pub struct HtmlRenderer{
    last_render: String
}
impl HtmlRenderer {
    pub fn new() -> Self {
        Self {
            last_render: String::new()
        }
    }
    pub fn get_render(&self) -> &str {
        &self.last_render
    }
}


#[derive(Debug)]
pub enum Tag {
    P(String),
    Div,
    Button
}
#[derive(Debug)]
pub struct HtmlTag{
    tag: Tag,
    attributes: Attributes
}
impl HtmlTag {
    pub fn write_open(&self, dest: &mut String) -> Result<(), std::fmt::Error>{
        match &self.tag {
            Tag::Div => {
                dest.write_str("<div")?;
                self.attributes.write(dest)?;
                dest.write_str(">")
            },
            Tag::Button => {
                dest.write_str("<button")?;
                self.attributes.write(dest)?;
                dest.write_str(">")
            }
            Tag::P(x) => {
                dest.write_str("<p")?;
                self.attributes.write(dest)?;
                dest.write_str(">")?;
                dest.write_str(x)
            }
        }
    }
    pub fn write_close(&self, dest: &mut String) -> Result<(), std::fmt::Error>{
        match &self.tag {
            Tag::Div => dest.write_str("</div>"),
            Tag::P(x) => dest.write_str("</p>"),
            Tag::Button => Ok(())
        }
    }
}


impl Renderer for HtmlRenderer {
    type RenderItem = HtmlTag;
    type QueryType<'gui> = BreadthIter<'gui>;
    fn render<'gui>(&mut self, q: visora_core::treecs::query::Query<'gui, &Self::RenderItem, Self::QueryType<'gui>>) {
        let mut out = String::new();
        for (info, tag) in q {
            match info {
                BreadthInfo::MoveUp => tag.write_close(&mut out).unwrap(),
                BreadthInfo::Other => tag.write_open(&mut out).unwrap()
            }
        }
        self.last_render = out;
    }
}

impl Render<Text> for HtmlRenderer {
    fn mount<'gui>(widget: &Text, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(HtmlTag{
            tag: Tag::P(widget.data.clone()),
            attributes: Attributes::new()
        });
    }
}

/*impl Render<Vlist<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Vlist<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(HtmlTag {
            tag: Tag::Div,
            attributes: Attributes::new()
        });
    }
}*/


impl Render<Hlist<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Hlist<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let attributes = Attributes::new()
            .with_style("display", Cow::Borrowed("flex"))
            .with_style("flex-direction", Cow::Borrowed("row"));
        context.mount_renderer(HtmlTag {
            tag: Tag::Div,
            attributes
        });
    }
}
impl Render<RichText> for HtmlRenderer {
    fn mount<'gui>(widget: &RichText, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let attributes = Attributes::new()
            .with_style("color", Cow::Owned(format!("#{:06X}", widget.color().value())));
        context.mount_renderer(HtmlTag {
            tag: Tag::P(widget.text().to_owned()),
            attributes
        });
    }
}
impl Render<Center<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Center<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let attributes = Attributes::new()
            .with_style("display", Cow::Borrowed("flex"))
            .with_style("justify-content", Cow::Borrowed("center"))
            .with_style("align-items", Cow::Borrowed("center"));
        context.mount_renderer(HtmlTag{
            tag: Tag::Div,
            attributes
        });
    }
}
/*impl Render<Container<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Container<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let mut attributes = Attributes::new();
        let padding = widget.insets();
        if padding.is_even() {
            attributes.add_style("padding", Cow::Owned(format!("{}px", padding.top())));
        } else {
            attributes.add_style("padding", Cow::Owned(format!("{}px {}px {}px {}px", padding.top(), padding.right(), padding.bottom(), padding.bottom())));
        }


        context.mount_renderer(HtmlTag {
            tag: Tag::Div,
            attributes
        });
    }
}*/

impl Render<TextButton> for HtmlRenderer {
    fn mount<'gui>(widget: &TextButton, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let attributes = Attributes::new()
            .with_on_click(Cow::Borrowed("socket.send('clicked')"));

        context.mount_renderer(HtmlTag {
            tag: Tag::Button,
            attributes
        });
    }
}
