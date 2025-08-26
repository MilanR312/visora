use std::{fs::{self, File}, io::Write};

use html::HtmlRenderer;
use visora::widget::{button::TextButton, center::Center, container::Container, list::Hlist, text::{RichText, Text, Vlist}};

use visora_core::{treecs::{iterators::breadth::BreadthInfo, query::Query}, widget::Render, Component, Gui};


visora::internal_impl!{Text, RichText, Vlist<Self>, Hlist<Self>, Center<Self>, Container<Self>, TextButton; ( $($bounds:tt)* ) => (
    pub trait SupportedWidgets: $($bounds)* + visora_core::renderer::Renderer {}
    impl<T: $($bounds)* + visora_core::renderer::Renderer> SupportedWidgets for T{}
)}


pub mod html;
pub mod server;
