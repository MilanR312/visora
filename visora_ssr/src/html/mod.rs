use std::{borrow::Cow, collections::HashMap, fmt::Write, fs::File, io::Write as ioWrite};

use itertools::Itertools;
use visora::widget::{button::Button, center::Center, container::Container, list::Hlist, text::{self, RichText, Text, Vlist}};
use visora_core::{renderer::Renderer, treecs::iterators::breadth::{BreadthInfo, BreadthIter}, widget::Render};

mod tags;



pub struct HtmlRenderer;


#[derive(Debug)]
pub enum Tag {
    P(String),
    Div,
    Button
}
#[derive(Debug)]
pub struct HtmlTag{
    tag: Tag,
    attributes: HashMap<&'static str, Cow<'static, str>>
}
impl HtmlTag {
    fn write_attributes(&self, dest: &mut String) -> Result<(), std::fmt::Error>{
        dest.write_str(" style=\"")?;
        let attrs = self.attributes.iter().map(|(key, val)| format!("{key}:{val}")).join(";");
        dest.write_str(&attrs)?;
        dest.write_str("\"")
    }
    pub fn write_open(&self, dest: &mut String) -> Result<(), std::fmt::Error>{
        match &self.tag {
            Tag::Div => {
                dest.write_str("<div")?;
                self.write_attributes(dest)?;
                dest.write_str(">")
            },
            Tag::Button => {
                dest.write_str("<button")?;
                self.write_attributes(dest)?;
                dest.write_str(">")
            }
            Tag::P(x) => {
                dest.write_str("<p")?;
                self.write_attributes(dest)?;
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
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("out.html").unwrap();
        file.write_all(out.as_bytes()).unwrap();
    }
}

impl Render<Text> for HtmlRenderer {
    fn mount<'gui>(widget: &Text, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(HtmlTag{
            tag: Tag::P(widget.data.clone()),
            attributes: HashMap::new()
        });
    }
}

impl Render<Vlist<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Vlist<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(HtmlTag {
            tag: Tag::Div,
            attributes: HashMap::new()
        });
    }
}
impl Render<Hlist<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Hlist<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let mut attributes = HashMap::new();
        attributes.insert("display", Cow::Borrowed("flex"));
        attributes.insert("flex-direction", Cow::Borrowed("row"));
        context.mount_renderer(HtmlTag {
            tag: Tag::Div,
            attributes
        });
    }
}
impl Render<RichText> for HtmlRenderer {
    fn mount<'gui>(widget: &RichText, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let mut attributes = HashMap::new();
        // TODO: add all attributes
        attributes.insert("color", Cow::Owned(format!("#{:06X}", widget.color().value())));
        dbg!(widget.color());
        context.mount_renderer(HtmlTag {
            tag: Tag::P(widget.text().to_owned()),
            attributes
        });
    }
}
impl Render<Center<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Center<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let mut attributes = HashMap::new();
        attributes.insert("display", Cow::Borrowed("flex"));
        attributes.insert("justify-content", Cow::Borrowed("center"));
        attributes.insert("align-items", Cow::Borrowed("center"));
        context.mount_renderer(HtmlTag{
            tag: Tag::Div,
            attributes
        });
    }
}
impl Render<Container<Self>> for HtmlRenderer {
    fn mount<'gui>(widget: &Container<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        let mut attributes = HashMap::new();
        let padding = widget.insets();
        if padding.is_even() {
            attributes.insert("padding", Cow::Owned(format!("{}px", padding.top())));
        } else {
            attributes.insert("padding", Cow::Owned(format!("{}px {}px {}px {}px", padding.top(), padding.right(), padding.bottom(), padding.bottom())));
        }


        context.mount_renderer(HtmlTag {
            tag: Tag::Div,
            attributes
        });
    }
}

impl Render<Button> for HtmlRenderer {
    fn mount<'gui>(widget: &Button, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(HtmlTag {
            tag: Tag::Button,
            attributes: HashMap::new()
        });
    }
}
