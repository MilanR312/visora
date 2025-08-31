
use std::{fs::File, io::Read, time::Duration};

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use tokio::{io::AsyncWriteExt, net::TcpListener, time::sleep};
use tokio_tungstenite::{accept_async, tungstenite::accept};
use visora::widget::{button::TextButton, list::Hlist, text::Text};
use visora_core::{Gui, WidgetContext, renderer::Renderer, state::State, widget::{RenderAble, Widget}};
use visora_macros::RenderAble;
use visora_ssr::html::HtmlRenderer;


/*#[derive(RenderAble)]
struct Name;

impl<R: visora_ssr::SupportedWidgets> Widget<R> for Name {
    type State = ();
    fn create_state(&self) -> Self::State {
        ()
    }
    fn build<'gui>(&self, state: &Self::State, context: &mut visora_core::BuildContext<'gui>) -> impl visora_core::widget::RenderAble<R> + 'static {
        Text::new("john doe")
    }
}*/


/*#[derive(RenderAble)]
struct Attribution{
    name: String
}
impl<R> Widget<R> for Attribution
where
    R: visora_ssr::SupportedWidgets,
{
    fn build<'gui>(
        &self,
        context: &mut visora_core::BuildContext<'gui>,
    ) -> impl Widget<R> + 'static {
        Center::new(
            Container::new()
                .with_child(
                    Vlist::new()
                        .add(Text::new("this is a working demo of modulars"))
                        .add(Hlist::new().add(Text::new("made by:")).add(Text::new(&self.name)))
                        .add(Text::lorem(50))
                        .add(
                            RichText::new("and in color :)".to_owned())
                                .with_color(Color::new_hex(0x0000ff)),
                        ),
                )
                .with_insets(EdgeInsets::all(20)),
        )
    }
}*/


//#[derive(RenderAble)]
struct Counter {
    start: u64,
    end: u64
}
impl<R> RenderAble<R> for Counter
where
    R: Renderer,
    Self: Widget<R>,
{

    fn mount<'gui>(
        &self,
        mut context: WidgetContext<'gui, R>,
    ) -> WidgetContext<'gui, R> {
        context.insert_component(self.create_state());
        let mut build_context = context.get_buildcontext();
        let state = State::new(context.get_buildcontext());
        let build = <Self as Widget<R>>::build(&self, state, &mut build_context);
        build.mount(context)
    }
}

impl<R: visora_ssr::SupportedWidgets> Widget<R> for Counter {
    type State = u64;
    fn create_state(&self) -> Self::State {
        self.start
    }
    
    fn build<'gui>(&self, state: State<Self>, context: &mut visora_core::BuildContext<'gui>) -> impl RenderAble<R> + 'static {
        Hlist::new()
            .add(
                Text::new(&format!("Counter is : {} [{}:{}]", state.read::<R>(), self.start, self.end))
            )
            .add(
                TextButton::new(Text::new("increment"))
                .on_click(state.update::<_, R>(|w, s| {
                    todo!()
                }))
            )
    }
}
// question:
// what to do when a method has an on_click? 
// Should State be owned and cheaply clonable?


#[tokio::main]
async fn main() {
    use visora_core::widget::{Widget};
    let wsserver = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    tokio::spawn(async {
        let server = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        let mut file = File::open("foo.html").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        loop {
            let (mut conn, _) = server.accept().await.unwrap();

            let response = format!(
                "HTTP/1.1 200 OK\r\n\
                Content-Length: {}\r\n\
                Content-Type: html\r\n\
                Connection: close\r\n\
                \r\n\
                {}",
                content.len(),
                content
            );
            conn.write_all(response.as_bytes()).await.unwrap();
        }
    });
    let mut gui = Gui::new(HtmlRenderer::new());
    let widget = Counter{start: 1, end: 5};
    let context = gui.root_widget_context();
    widget.mount(context);
    gui.render();
    let rendered = gui.renderer().get_render();
    loop {
        let rendered = rendered.to_owned();
        let (x, _) = wsserver.accept().await.unwrap();
        println!("got ws connection");
        tokio::spawn(async move {
            let mut ws = accept_async(x).await.unwrap();
            let (mut sender, mut receiver) = ws.split();

            loop {
                sender.send(format!("replace|root|{}", rendered).into()).await.unwrap();
                let x = receiver.next().await;
                println!("{x:?}");
                sleep(Duration::from_secs(1)).await;
            }
        });
    }
    /*

    let server = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    loop {
        let (mut conn, _) = server.accept().await.unwrap();
        gui.render();
        let rendered = gui.renderer().get_render();
        let response = Response::builder()
            .status(StatusCode::OK)
            .body(rendered)
            .unwrap();

    }
    println!("got connection");
    loop {
        gui.render();
        let rendered = gui.renderer().get_render();
        dbg!(rendered);
        conn.write_all(rendered.as_bytes()).await.unwrap();
        break;
    }*/
}
