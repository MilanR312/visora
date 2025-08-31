use std::{fs::{self, File}, io::Write};

use html::HtmlRenderer;

use visora::widget::{button::TextButton, center::Center, text::{RichText, Text}, list::Hlist};
use visora_core::{treecs::{iterators::breadth::BreadthInfo, query::Query}, widget::Render, Component, Gui};


visora::internal_impl!{
    Text, 
    TextButton,
    Hlist<Self>,
    RichText, 
    Center<Self>;
    ( $($bounds:tt)* ) => (
    pub trait SupportedWidgets: $($bounds)* + visora_core::renderer::Renderer {}
    impl<T: $($bounds)* + visora_core::renderer::Renderer> SupportedWidgets for T{}
)}


pub mod html;
