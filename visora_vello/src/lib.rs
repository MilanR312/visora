use std::{collections::{HashMap, VecDeque}, fmt::Display, ops::Sub, path::PathBuf, sync::{Arc, LazyLock}};

use image::DynamicImage;
use visora::widget::{self, container::EdgeInsets};
use visora_core::{color, renderer, treecs::iterators::breadth::{BreadthInfo, BreadthIter, Reversed}, widget::Render};
use vello::{kurbo::{Affine, Rect, Vec2}, peniko::{self, Blob, Brush, Color, Font, Style}, skrifa::{prelude::Size as SSize, FontRef, MetadataProvider}, Glyph, Scene};
use winit::window::Window;

pub mod runner;


pub struct ModulaRenderer{
    pub scene: Scene,
    pub font: Font,
    pub window: Option<Arc<Window>>
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Constraint {
    width: f32,
    height: f32
}
impl Sub<Constraint> for Constraint {
    type Output = Constraint;
    fn sub(self, rhs: Constraint) -> Self::Output {
        Self {
            height: self.height - rhs.height,
            width: self.width - rhs.width
        }
    }
}
impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
pub type Size = Constraint;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Area {
    top: f32,
    left: f32,
    width: f32,
    height: f32,
}
impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.left, self.top, self.width, self.height)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical
}
/*
pub trait Drawable{
    fn pass_constraint(&self, constraint: Constraint) -> Constraint;
    fn get_area(&self, self_constraint: Constraints, child_size: Option<Size>, renderer: &ModulaRenderer) -> Size;
    fn cut_direction(&self) -> Option<Direction> {
        None
    }
    fn draw(&self, area: Area, renderer: &mut ModulaRenderer);
    fn get_child_area(&self, own_area: Area, child_size: Size) -> Area {
        own_area
    }
}


pub struct Text{
    data: String,
    fontsize: f32
}
impl Drawable for Text {
    fn pass_constraint(&self, constraint: Constraint) -> Constraint {
        constraint
    }
    fn get_area(&self, self_constraint: Constraints, child_size: Option<Size>, renderer: &ModulaRenderer) -> Size {
        let fontref = to_font_ref(&renderer.font).unwrap();
        let fontsize = SSize::new(self.fontsize);
        let axes = fontref.axes();
        let var_loc = axes.location::<&[(&str, f32)]>(&[]);
        let metrics = fontref.metrics(fontsize, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = fontref.glyph_metrics(fontsize, &var_loc);
        let charmap = fontref.charmap();

        let mut current_width = 0.0;
        let mut max_width = 0.0;
        let mut current_height = 0.0;
        for x in self.data.chars() {
            let gid = charmap.map(x).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
            current_width += advance;
            if current_width > self_constraint.modified.width {
                max_width = self_constraint.modified.width;
                current_width = 0.0;
                current_height += line_height;
            }
        }
        current_height += line_height; // since we added another line
        Constraint { 
            height: current_height,
            width: max_width
        }
    }
    fn draw(&self, area: Area, renderer: &mut ModulaRenderer) {
        let fontsize = SSize::new(self.fontsize);
        let fontref = to_font_ref(&renderer.font).unwrap();
        let axes = fontref.axes();
        let var_loc = axes.location::<&[(&str, f32)]>(&[]);
        let metrics = fontref.metrics(fontsize, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = fontref.glyph_metrics(fontsize, &var_loc);
        let charmap = fontref.charmap();
        let style = Style::Fill(vello::peniko::Fill::NonZero);
        let mut pen_y = 0.0;
        let mut pen_x = 0.0;
        let offset = Affine::IDENTITY.with_translation(Vec2::new(area.left as f64, (area.top as f64) + line_height  as f64));
        renderer.scene
            .draw_glyphs(&renderer.font)
            .transform(offset)
            .font_size(self.fontsize)
            //.normalized_coords(var_loc.coords())
            .brush(&Brush::Solid(Color::WHITE))
            .draw(
                &style,
                self.data.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = 0.0;
                        return None;
                    }
                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    if pen_x + advance >= area.width as f32 {
                        pen_x = 0.0;
                        pen_y += line_height;
                    }
                    let x = pen_x;
                    pen_x += advance;
                    Some(Glyph {
                        id: gid.to_u32(),
                        x,
                        y: pen_y,
                    })
                }),
            );
    }
}


pub struct Container {
    insets: EdgeInsets,
    bg: color::Color
}
impl Drawable for Container {
    fn pass_constraint(&self, constraint: Constraint) -> Constraint {
        let insets = self.insets;
        Constraint {
            height: constraint.height - (insets.top() + insets.bottom()) as f32,
            width: constraint.width - (insets.left() + insets.right()) as f32
        }
    }
    fn get_child_area(&self, own_area: Area, child_size: Size) -> Area {
        Area {
            top: own_area.top + self.insets.top() as f32,
            left: own_area.left + self.insets.left() as f32,
            height: child_size.height,
            width: child_size.width
        }
    }
    fn get_area(&self, self_constraint: Constraints, child_size: Option<Size>, renderer: &ModulaRenderer) -> Size {
        let child_size = child_size.unwrap();
        Size {
            height: child_size.height + (self.insets.top() + self.insets.bottom()) as f32,
            width: child_size.width + (self.insets.left() + self.insets.right()) as f32
        }        
    }
    fn draw(&self, area: Area, renderer: &mut ModulaRenderer) {
        let brush = Brush::Solid(Color::rgba8(self.bg.red(), self.bg.green(), self.bg.blue(), self.bg.alpha()));
        let rect = Rect::new(area.left as f64, area.top as f64, (area.left + area.width) as f64, (area.top + area.height) as f64);
        let fill = vello::peniko::Fill::NonZero;
        renderer.scene
            .fill(fill, Affine::IDENTITY, &brush, None, &rect);
    }
}

pub struct Centered;
impl Drawable for Centered {
    fn pass_constraint(&self, constraint: Constraint) -> Constraint {
        constraint
    }
    fn get_area(&self, self_constraint: Constraints, child_size: Option<Size>, renderer: &ModulaRenderer) -> Size {
        self_constraint.original
    }
    fn draw(&self, area: Area, renderer: &mut ModulaRenderer) {
        // nothing to draw
    }
    fn get_child_area(&self, own_area: Area, child_size: Size) -> Area {
        let child_left = own_area.left + own_area.width / 2.0 - child_size.width / 2.0;
        let child_top = own_area.top + own_area.height / 2.0 - child_size.height / 2.0;
        Area {
            left: child_left,
            top: child_top,
            height: child_size.height,
            width: child_size.width
        }
    }
}


pub struct List{
    direction: Direction
}
impl Drawable for List {
    fn cut_direction(&self) -> Option<Direction> {
        Some(self.direction)
    }
    fn draw(&self, area: Area, renderer: &mut ModulaRenderer) {
        
    }
    fn get_area(&self, self_constraint: Constraints, child_size: Option<Size>, renderer: &ModulaRenderer) -> Size {
        child_size.unwrap_or(self_constraint.original)
    }
    fn get_child_area(&self, own_area: Area, child_size: Size) -> Area {
        own_area
    }
    fn pass_constraint(&self, constraint: Constraint) -> Constraint {
        constraint
    }
}

pub enum Element {
    Text(Text),
    Container(Container),
    Centered,
    List(List)
}
impl Drawable for Element {
    fn get_area(&self, self_constraint: Constraints, child_size: Option<Size>, renderer: &ModulaRenderer) -> Size {
        match self {
            Self::Container(x) => x.get_area(self_constraint, child_size, renderer),
            Self::Text(x) => x.get_area(self_constraint, child_size, renderer),
            Self::Centered => Centered.get_area(self_constraint, child_size, renderer),
            Self::List(x) => x.get_area(self_constraint, child_size, renderer)
        }
    }
    fn pass_constraint(&self, constraint: Constraint) -> Constraint{
        match self {
            Self::Container(x) => x.pass_constraint(constraint),
            Self::Text(x) => x.pass_constraint(constraint),
            Self::Centered => Centered.pass_constraint(constraint),
            Self::List(x) => x.pass_constraint(constraint)
        }
    }
    fn draw(&self, area: Area, renderer: &mut ModulaRenderer) {
        match self {
            Self::Centered => Centered.draw(area, renderer),
            Self::Container(x) => x.draw(area, renderer),
            Self::Text(x) => x.draw(area, renderer),
            Self::List(x) => x.draw(area, renderer),
        }
    }
    fn get_child_area(&self, own_area: Area, child_size: Size) -> Area {
        match self {
            Self::Centered => Centered.get_child_area(own_area, child_size),
            Self::Container(x) => x.get_child_area(own_area, child_size),
            Self::Text(x) => x.get_child_area(own_area, child_size),
            Self::List(x) => x.get_child_area(own_area, child_size)
        }
    }
}
impl Element {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Container(_) => "container",
            Self::Text(_) => "text",
            Self::Centered => "centered",
            Self::List(_) => "list"
        }
    }
}*/

pub trait Drawable: Send+ Sync {
    fn name(&self) -> &'static str;
    /// get the constraint to push to the constraint stack
    fn constraint(&self, parent: Constraints, renderer: &ModulaRenderer) -> Option<Constraint>;
    fn calc_size(&self, parent: Constraints, renderer: &ModulaRenderer) -> Size;
    fn update_parent(&self, self_size: Size, parent: &mut Constraints, renderer: &ModulaRenderer);
    fn lays_constraint(&self) -> bool;
    fn generate_child_area(&self, working_area: &mut Area, child_size: Size) -> Option<Area>{
        None
    }
    fn draw(&self, renderer: &mut ModulaRenderer, area: Area);
}

pub type Element = Box<dyn Drawable>;
pub struct Text {
    data: String,
    fontsize: f32,
    bold: bool
}
impl Drawable for Text {
    fn name(&self) -> &'static str {
        "text"
    }
    fn lays_constraint(&self) -> bool {
        false
    }
    fn calc_size(&self, parent: Constraints, renderer: &ModulaRenderer) -> Size {
        let work_area = parent.remaining;
        //let fontref = to_font_ref( if self.bold { &renderer.font_bold } else { &renderer.font }).unwrap();
        let fontref = to_font_ref(&renderer.font).unwrap();
        
        let fontsize = SSize::new(self.fontsize);
        let axes = fontref.axes();
        let var_loc = if self.bold {
            axes.location(&[("wght", 700.0)])
        } else {
            axes.location(&[("wght", 300.0)])
        };
        let metrics = fontref.metrics(fontsize, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = fontref.glyph_metrics(fontsize, &var_loc);
        let charmap = fontref.charmap();

        let mut current_width = 0.0;
        let mut max_width = 0.0;
        let mut current_height = 0.0;
        for x in self.data.chars() {
            let gid = charmap.map(x).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
            current_width += advance;
            if current_width > work_area.width {
                max_width = work_area.width;
                current_width = 0.0;
                current_height += line_height;
            }
        }
        if max_width == 0.0 {
            max_width = current_width + 1.0; //TODO: this should be better
        }
        current_height += line_height; // since we added another line
        Constraint { 
            height: current_height,
            width: max_width
        }
        /*Constraint {
            height: if parent.remaining.width == 70.0 { 20.0 } else { 10.0 },
            width: parent.remaining.width
        }*/
    }


    fn constraint(&self, parent: Constraints, renderer: &ModulaRenderer) -> Option<Constraint> {
        None
    }
    fn update_parent(&self, self_size: Size, parent: &mut Constraints, renderer: &ModulaRenderer) {
        parent.remaining.height -= self_size.height;
        if parent.remaining.width == parent.original.width {
            parent.remaining.width = self_size.width;
        } else {
            parent.remaining.width = self_size.width.max(parent.remaining.width);
        }
    }
    fn draw(&self, renderer: &mut ModulaRenderer, area: Area) {
        /*println!("drawn text at {area:?}");
        let brush = Brush::Solid(Color::rgba8(255, 0, 0, 255));
        let rect = Rect::new(area.left as f64, area.top as f64, (area.left + area.width) as f64, (area.top + area.height) as f64);
        let fill = vello::peniko::Fill::NonZero;
        renderer.scene
            .fill(fill, Affine::IDENTITY, &brush, None, &rect);
        */
        let fontref = to_font_ref(&renderer.font).unwrap();

        let fontsize = SSize::new(self.fontsize);
        let axes = fontref.axes();
        let var_loc = if self.bold {
            axes.location([("wght", 700.0)])
        } else {
            axes.location(&[("wght", 300.0)])

        };
        println!("axes = {var_loc:?}");
        let metrics = fontref.metrics(fontsize, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = fontref.glyph_metrics(fontsize, &var_loc);
        let charmap = fontref.charmap();

        let style = Style::Fill(vello::peniko::Fill::NonZero);
        let mut pen_y = 0.0;
        let mut pen_x = 0.0;
        let offset = Affine::IDENTITY.with_translation(Vec2::new(area.left as f64, (area.top as f64) + line_height  as f64));
        renderer.scene
            .draw_glyphs(&renderer.font)
            .transform(offset)
            .font_size(self.fontsize)
            //.normalized_coords(var_loc.coords())
            .brush(&Brush::Solid(Color::BLACK))
            .draw(
                &style,
                self.data.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = 0.0;
                        return None;
                    }
                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    if pen_x + advance >= area.width as f32 {
                        pen_x = 0.0;
                        pen_y += line_height;
                    }
                    let x = pen_x;
                    pen_x += advance;
                    Some(Glyph {
                        id: gid.to_u32(),
                        x,
                        y: pen_y,
                    })
                }),
            );
    }
}

pub enum ImageError{
    FileNotFound,
    InvalidFormat
}

static IMG_NOT_FOUND: LazyLock<DynamicImage> = LazyLock::new(|| {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("../img_not_found.png");
    let data = image::ImageReader::open(path).unwrap();
    let data = data.decode().unwrap();
    data
});
static IMG_NOT_RECOGNISED: LazyLock<DynamicImage> = LazyLock::new(|| {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("../img_not_found.png");
    let data = image::ImageReader::open(path).unwrap();
    let data = data.decode().unwrap();
    data
});

pub struct Image{
    data: Result<DynamicImage, ImageError>,
    max_width: Option<u32>,
    max_height: Option<u32>
}
impl Image {
    fn err(err: ImageError, max_width: Option<u32>, max_height: Option<u32>) -> Self {
        Self {
            data: Err(err),
            max_height,
            max_width
        }
    }
    pub fn new(path: PathBuf, max_width: Option<u32>, max_height: Option<u32>) -> Self {
        let data = match image::ImageReader::open(path){
            Ok(x) => x,
            Err(e) => return Self::err(ImageError::FileNotFound, max_width, max_height)
        };
        let data = match data.decode(){
            Ok(x) => x,
            Err(e) => return Self::err(ImageError::InvalidFormat, max_width, max_height)
        };
        Self{
            data: Ok(data),
            max_height,
            max_width
        }
    }
    pub fn generate_image(&self, size: Size) -> peniko::Image {
        let width = size.width as u32;
        let height = size.height as u32;
        let width = self.max_width.map_or_else(|| width, |x| x.min(width));
        let height = self.max_height.map_or_else(|| height, |x| x.min(height));
        let data = match &self.data {
            Ok(x) => x.resize(width, height, image::imageops::FilterType::Gaussian),
            Err(x) => match x {
                ImageError::FileNotFound => IMG_NOT_FOUND.resize(width, height, image::imageops::FilterType::Gaussian),
                ImageError::InvalidFormat => IMG_NOT_RECOGNISED.resize(width, height, image::imageops::FilterType::Gaussian)
            }
        };
        let raw_data = data.to_rgba8().to_vec().into_boxed_slice();
        let blob = Blob::new(Arc::new(raw_data));
        peniko::Image::new(blob, peniko::Format::Rgba8, width, height)
    }
}
/*
impl Drawable for Image {
    fn name(&self) -> &'static str {
        "image"
    }
    fn lays_constraint(&self) -> bool {
        false
    }
    fn constraint(&self, parent: Constraints, renderer: &ModulaRenderer) -> Option<Constraint> {
        None
    }
    fn calc_size(&self, parent: Constraints, renderer: &ModulaRenderer) -> Size {
        // image is a growable widget and takes in as much space as it can
        parent.remaining
    }
    fn update_parent(&self, self_size: Size, parent: &mut Constraints, renderer: &ModulaRenderer) {
        parent.remaining
    }
    fn generate_child_area(&self, working_area: &mut Area, child_size: Size) -> Option<Area> {
        None
    }
    fn draw(&self, renderer: &mut ModulaRenderer, area: Area) {
        let img = self.generate_image(Size{ height: area.height, width: area.width });
        renderer.scene.draw_image(&img, Affine::translate(Vec2::new(area.left as f64, area.top as f64)));
    }
}*/

pub struct Container {
    insets: EdgeInsets,
    bg: color::Color
}
impl Drawable for Container {
    fn name(&self) -> &'static str {
        "container"
    }
    fn lays_constraint(&self) -> bool {
        true
    }
    fn constraint(&self, parent: Constraints, renderer: &ModulaRenderer) -> Option<Constraint> {
        let a = Some(Constraint {
            height: parent.remaining.height - (self.insets.top() + self.insets.bottom()) as f32,
            width: parent.remaining.width - (self.insets.left() + self.insets.right()) as f32
        });
        //println!("c: {parent} -> {a:?}");
        a
    }
    fn calc_size(&self, parent: Constraints, renderer: &ModulaRenderer) -> Size {
        // parent in this context is the constraint the widget itself laid upon the child
        let work_area = parent.work_area();
        Size {
            width: parent.remaining.width + (self.insets.left() + self.insets.right()) as f32,
            height: work_area.height + (self.insets.top() + self.insets.bottom()) as f32
        }
    }
    fn update_parent(&self, self_size: Size, parent: &mut Constraints, renderer: &ModulaRenderer) {
        parent.remaining.height -= self_size.height;
        if parent.remaining.width == parent.original.width {
            parent.remaining.width = self_size.width;
        } else {
            parent.remaining.width = self_size.width.max(parent.remaining.width);
        }
        // not needed, when we are in this widget we pop from the constraint stack and this updates the parent anyway
    }
    fn generate_child_area(&self, working_area: &mut Area, child_size: Size) -> Option<Area> {
        Some(Area {
            top: working_area.top + self.insets.top() as f32,
            left: working_area.left + self.insets.left() as f32,
            width: working_area.width - (self.insets.left() + self.insets.right()) as f32,
            height: working_area.height - (self.insets.top() + self.insets.bottom()) as f32
        })
    }
    fn draw(&self, renderer: &mut ModulaRenderer, area: Area) {
        let brush = Brush::Solid(Color::rgba8(self.bg.red(), self.bg.green(), self.bg.blue(), self.bg.alpha()));
        let rect = Rect::new(area.left as f64, area.top as f64, (area.left + area.width) as f64, (area.top + area.height) as f64);
        let fill = vello::peniko::Fill::NonZero;
        renderer.scene
            .fill(fill, Affine::IDENTITY, &brush, None, &rect);
    }
}
pub struct List{
    dir: Direction
}
impl Drawable for List {
    fn name(&self) -> &'static str {
        "list"
    }
    fn lays_constraint(&self) -> bool {
        false
    }
    fn calc_size(&self, parent: Constraints, renderer: &ModulaRenderer) -> Size {
        Size {
            width: parent.remaining.width,
            height: parent.original.height - parent.remaining.height
        }
    }
    fn constraint(&self, parent: Constraints, renderer: &ModulaRenderer) -> Option<Constraint> {
        None
    }
    fn update_parent(&self, self_size: Size, parent: &mut Constraints, renderer: &ModulaRenderer) {
        // TODO: does this need something?
    }
    fn generate_child_area(&self, working_area: &mut Area, child_size: Size) -> Option<Area> {
        let out = Area {
            top: working_area.top,
            left: working_area.left,
            width: child_size.width,
            height: child_size.height
        };
        working_area.top += child_size.height;
        working_area.height -= child_size.height;
        Some(out)
    }
    fn draw(&self, renderer: &mut ModulaRenderer, area: Area) {
        // a list doesnt draw and only reserves space
    }
}
pub struct Center;
impl Drawable for Center {
    fn name(&self) -> &'static str {
        "center"
    }
    fn lays_constraint(&self) -> bool {
        true
    }
    fn constraint(&self, parent: Constraints, renderer: &ModulaRenderer) -> Option<Constraint> {
        Some(parent.remaining)
    }
    fn calc_size(&self, parent: Constraints, renderer: &ModulaRenderer) -> Size {
        parent.remaining
    }
    fn update_parent(&self, self_size: Size, parent: &mut Constraints, renderer: &ModulaRenderer) {
        // TODO: check if this shouldnt do anything
    }
    fn generate_child_area(&self, working_area: &mut Area, child_size: Size) -> Option<Area> {
        let child_top = working_area.height / 2.0 - child_size.height / 2.0;
        let child_left = working_area.width / 2.0 - child_size.width / 2.0;
        Some(Area {
            height: child_size.height,
            width: child_size.width,
            top: child_top,
            left: child_left
        })
    }
    fn draw(&self, renderer: &mut ModulaRenderer, area: Area) {
        
    }
}


#[derive(Clone, Copy)]
pub struct Constraints {
    original: Constraint,
    remaining: Constraint
}
impl Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{og: {},rem: {} }}", self.original, self.remaining)
    }
}
impl Constraints {
    pub fn new(constraint: Constraint) -> Self {
        Self{
            original: constraint,
            remaining: constraint
        }
    }
    pub fn work_area(&self) -> Constraint {
        self.original - self.remaining
    }
}

impl ModulaRenderer {
    pub fn generate_sizes<'gui>(&self, window: &Window, q: &mut visora_core::treecs::query::Query<'gui, &<Self as renderer::Renderer>::RenderItem, <Self as renderer::Renderer>::QueryType<'gui>>) -> VecDeque<Size>{
        let mut constraints = Vec::new();
        constraints.push(Constraints::new(Constraint{
            width: window.outer_size().width as f32,
            height: window.outer_size().height as f32
            //width: 100.0,
            //height: 100.0
        }));
        //let mut cut_directions = Vec::new();
        let mut sizes = VecDeque::new();
        println!("\nsizes");
        
        while let Some((info, ell)) = q.next() {
            println!("{} {}: ", ell.name(), if info == BreadthInfo::MoveUp { '^' } else { 'V' });
            println!("\t{}", constraints.last().unwrap());
            match info {
                BreadthInfo::Other => {
                    let parent_constraint = *constraints.last().unwrap();
                    if let Some(x) = ell.constraint(parent_constraint, self){
                        println!("\tpush({x})");
                        constraints.push(Constraints::new(x));
                    }
                    // when going down we always push the childs constraint
                    /*let self_constraint = constraints.last().unwrap();
                    
                    let constraint_of_child = ell.pass_constraint(self_constraint.modified);
                    println!("c {}: self {} child: {}", ell.name(), self_constraint.modified, constraint_of_child);
                    constraints.push(Constraints::new(constraint_of_child));
                    */
                    /*if let Some(x) = ell.cut_direction() {
                        cut_directions.push(x);
                    }*/
                },
                BreadthInfo::MoveUp => {
                    let parent_constraint = *constraints.last().unwrap();
                    let size: Size = ell.calc_size(parent_constraint, self);
                    println!("\tcalc_size({}) == {}", parent_constraint, size);
                    //print!("p {} {}", *parent_constraint, size);
                    if ell.lays_constraint() {
                        let a = constraints.pop();
                        println!("\tpop({}) ", a.unwrap())
                    }
                    let p_before = parent_constraint;
                    let parent_constraint = constraints.last_mut().unwrap();
                    ell.update_parent(size, parent_constraint, self);
                    println!("\tupdate parent {p_before} -> {parent_constraint}");                  
                    sizes.push_back(size);

                }
            }
        }

        sizes
    }
}
impl renderer::Renderer for ModulaRenderer {
    type QueryType<'gui> = BreadthIter<'gui, Reversed>;
    type RenderItem = Element;
    fn render<'gui>(&mut self, mut q: visora_core::treecs::query::Query<'gui, &Self::RenderItem, Self::QueryType<'gui>>) {
        let Some(x) = &self.window else { return ; };
        let mut sizes = self.generate_sizes(&x, &mut q);
        let mut area_stack = Vec::new();
        area_stack.push(Area{
            top: 0.0,
            left: 0.0,
            height: x.outer_size().height as f32,
            width: x.outer_size().width as f32
        });
        /*area_stack.push(Area{
            top: 0.0,
            left: 0.0,
            height: 100.0,
            width: 100.0
        });*/
        println!("");
        println!("astack = {area_stack:?}");
        println!("sizestack = {sizes:?}");
        //sizes.pop_back();
        let q = q.restart().reverse();
        for (info, ell) in q {
            match info {
                BreadthInfo::Other => {
                    println!("{}: ", ell.name());
                    println!("\tself_area: {}", area_stack.last().unwrap());
                    println!("\tself_size: {:?}", sizes.back().unwrap());
                    let self_size = sizes.pop_back().unwrap();
                    let self_area = *area_stack.last_mut().unwrap();
                    if let Some(child_size) = sizes.back(){
                        let self_area = area_stack.last_mut().unwrap();
                        let child_area = ell.generate_child_area(self_area, *child_size);
                        if let Some(child_area) = child_area {
                            area_stack.push(child_area);
                        }
                        println!("\tchild_area: {child_area:?}");
                    }
                    ell.draw(self, self_area);
                    //let child_area = sizes.pop_back().unwrap();
                    //let area = ell.generate_child_area(self_area, child_area);
                    //println!("area: {area:?}");
                    /*if let Some(c_size) = sizes.pop_back() {
                        let area = ell.get_child_area(self_area, c_size);
                        println!("{}: sa {} cs {} ca {}", ell.name(), self_area, c_size, area);
                        area_stack.push(area);
                    }*/
                    //println!("draw {} at {}", ell.name(), self_area);
                    //ell.draw(self_area, self);
                },
                BreadthInfo::MoveUp => {
                    area_stack.pop();
                }
            }
        }
    }
}


impl Render<widget::text::Text> for ModulaRenderer {
    fn mount<'gui>(widget: &widget::text::Text, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Box::new(Text {
            data: widget.data.clone(),
            fontsize: 25.0,
            bold: widget.is_bold
        }))
    }
}
impl Render<widget::container::Container<Self>> for ModulaRenderer {
    fn mount<'gui>(widget: &widget::container::Container<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Box::new(Container {
            bg: *widget.bg(),
            insets: *widget.insets()
        }));
    }
}
impl Render<widget::text::Vlist<Self>> for ModulaRenderer {
    fn mount<'gui>(widget: &widget::text::Vlist<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Box::new(List {
            dir: Direction::Vertical
        }));
    }
}
impl Render<widget::center::Center<Self>> for ModulaRenderer {
    fn mount<'gui>(widget: &widget::center::Center<Self>, context: &mut visora_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Box::new(Center));
    }
}
/*
impl Render<widget::image::Image> for ModulaRenderer {
    fn mount<'gui>(widget: &widget::image::Image, context: &mut modula_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Box::new(Image::new(widget.path.clone(), widget.max_width, widget.max_height)));
    }
}*/
/*
impl Render<Vlist<Self>> for ModulaRenderer {
    fn mount<'gui>(widget: &Vlist<Self>, context: &mut modula_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Drawable::List { direction: true });
    }
}

impl Render<widget::center::Center<Self>> for ModulaRenderer {
    fn mount<'gui>(widget: &widget::center::Center<Self>, context: &mut modula_core::WidgetContext<'gui, Self>) {
        context.mount_renderer(Element::Centered);
    }
}

*/

fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    use vello::skrifa::raw::FileRef;
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}