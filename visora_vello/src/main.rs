// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT




use std::sync::Arc;

use vello::{peniko::{Blob, Font}, Scene};
use visora::widget::{center::Center, container::Container, image::Image, list::Hlist, text::{Text, Vlist}};
use visora_core::{color::Color, renderer::Renderer, widget::{Render, StatelessWidget, Widget}};
use visora_macros::StatelessWidget;
use visora_vello::{runner::run_app, ModulaRenderer};


#[derive(StatelessWidget)]
struct App;
impl<R: Renderer> StatelessWidget<R> for App
where R: Render<Container<R>> + Render<Text> + Render<Hlist<R>> + Render<Center<R>> + Render<Image>//+ Render<Center<R>> + Render<Vlist<R>>
{
    fn build<'gui>(&self, context: &mut visora_core::BuildContext<'gui>) -> impl Widget<R> + 'static {
        Container::new()
            .with_child(
                Center::new(
                    Hlist::new()
                    .add(
                        Image::new("ferris.png")
                        .with_max_height(200)
                        .with_max_width(200)
                        //Text::new("test")  
                    )
                    .add(
                        Image::new("ferris.png")
                        .with_max_height(200)
                        .with_max_width(200)
                        //Text::new("test")  
                    )
                )
            )
            .with_bg(Color::new_hex(0xffffffff))
        }
    }
    
//const FONT: &[u8] = include_bytes!("/usr/share/fonts/cantarell/Cantarell-VF.otf");
    
fn main(){
    let path = "/usr/share/fonts/cantarell/Cantarell-VF.otf";
    //let path = "/usr/share/fonts/opentype/urw-base35/NimbusRoman-Regular.otf";
    let data = std::fs::read(path).unwrap();

    // Setup a bunch of state:
    let font = Font::new(Blob::new(Arc::new(data.into_boxed_slice())), 0);
    let renderer = ModulaRenderer {
        font,
        scene: Scene::new(),
        window: None
    };
    run_app(renderer, App, |x|{});
}



