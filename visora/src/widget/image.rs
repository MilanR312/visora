use std::path::{Path, PathBuf};

use visora_core::{renderer::Renderer, widget::{Render, Widget}, WidgetContext};


pub struct Image{
    // an image widget is only a path, this keeps it cheap
    pub path: PathBuf,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>
}
impl Image {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        let path = path.as_ref().to_path_buf();
        Self {
            path,
            max_height: None,
            max_width: None
        }
    }
    pub fn with_max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }
    pub fn with_max_height(mut self, height: u32) -> Self {
        self.max_height = Some(height);
        self
    }
}
impl<R> Widget<R> for Image
where R: Renderer + Render<Self>
{
    fn mount<'gui>(&self, mut context: WidgetContext<'gui, R>) -> WidgetContext<'gui, R> {
        R::mount(self, &mut context);
        context
    }
}