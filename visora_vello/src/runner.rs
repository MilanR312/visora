use eyre::Result;
use visora::widget::center::Center;
use visora::widget::container::{Container, EdgeInsets};
use visora::widget::image::Image;
use visora::widget::text::{Text, Vlist};
use visora_core::{color, renderer, Gui};
use visora_core::treecs::iterators::breadth::{BreadthInfo, BreadthIter, Reversed};
use visora_core::widget::{Render, StatelessWidget, Widget};
use visora_macros::StatelessWidget;
use crate::ModulaRenderer;
use vello::skrifa::prelude::Size;
use vello::skrifa::{FontRef, MetadataProvider};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;
use vello::kurbo::{Affine, Circle, Ellipse, Line, Rect, RoundedRect, Stroke, Vec2};
use vello::peniko::{Blob, Brush, Color, Font, Style};
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Glyph, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;

use vello::wgpu;


pub struct ActiveRenderState<'s> {
    // The fields MUST be in this order, so that the surface is dropped before the window
    surface: RenderSurface<'s>,
    window: Arc<Window>,
}

enum RenderState<'s> {
    Active(ActiveRenderState<'s>),
    // Cache a window so that it can be reused when the app is resumed after being suspended
    Suspended(Option<Arc<Window>>),
}

struct AppRunner<'s, App, F> {
    // The vello RenderContext which is a global context that lasts for the
    // lifetime of the application
    context: RenderContext,

    // An array of renderers, one per wgpu device
    renderers: Vec<Option<Renderer>>,

    // State for our example where we store the winit Window and the wgpu Surface
    state: RenderState<'s>,

    gui: Gui<ModulaRenderer>,
    widget: App,
    set_window: F,
    name: Cow<'static, str>
}

impl<'s, App: Widget<ModulaRenderer>, F: Fn(Arc<Window>)> ApplicationHandler for AppRunner<'s, App, F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        // Get the winit window cached in a previous Suspended event or else create a new window
        let window = cached_window
            .take()
            .unwrap_or_else(|| create_winit_window(event_loop, self.name.as_ref()));
        (self.set_window)(window.clone());
        self.gui.renderer().window = Some(window.clone());
        // Create a vello Surface
        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Create a vello Renderer for the surface (using its device id)
        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id]
            .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

        // Save the Window and Surface to a state variable
        self.state = RenderState::Active(ActiveRenderState { window, surface });
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active(state) = &self.state {
            self.state = RenderState::Suspended(Some(state.window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Ignore the event (return from the function) if
        //   - we have no render_state
        //   - OR the window id of the event doesn't match the window id of our render_state
        //
        // Else extract a mutable reference to the render state from its containing option for use below
        let render_state = match &mut self.state {
            RenderState::Active(state) if state.window.id() == window_id => state,
            _ => return,
        };

        match event {
            // Exit the event loop when a close is requested (e.g. window's close button is pressed)
            WindowEvent::CloseRequested => event_loop.exit(),

            // Resize the surface when the window is resized
            WindowEvent::Resized(size) => {
                self.context
                    .resize_surface(&mut render_state.surface, size.width, size.height);
            }
            // This is where all the rendering happens
            WindowEvent::RedrawRequested => {
                //temp since statefullwidgets dont exists yet
                self.gui.clear_tree();
                let context = self.gui.root_widget_context();
                self.widget.mount(context);
                // Empty the scene of objects to draw. You could create a new Scene each time, but in this case
                // the same Scene is reused so that the underlying memory allocation can also be reused.
                self.gui.renderer().scene.reset();
                self.gui.render();

                // Get the RenderSurface (surface + config)
                let surface = &render_state.surface;

                // Get the window size
                let width = surface.config.width;
                let height = surface.config.height;

                // Get a handle to the device
                let device_handle = &self.context.devices[surface.dev_id];

                // Get the surface's texture
                let surface_texture = surface
                    .surface
                    .get_current_texture()
                    .expect("failed to get surface texture");

                // Render to the surface's texture
                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_surface(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.gui.renderer().scene,
                        &surface_texture,
                        &vello::RenderParams {
                            base_color: Color::BLACK, // Background color
                            width,
                            height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .expect("failed to render to surface");

                // Queue the texture to be presented on the surface
                surface_texture.present();

                device_handle.device.poll(wgpu::Maintain::Poll);
            }
            _ => {}
        }
    }
}

pub fn run_app<T: Into<Cow<'static, str>>>(renderer: ModulaRenderer, x: impl Widget<ModulaRenderer>, name: T,set_window: impl Fn(Arc<Window>)){
    let mut gui = Gui::new(renderer);
    let context = gui.root_widget_context();
    x.mount(context);

    let mut app = AppRunner {
        context: RenderContext::new(),
        renderers: vec![],
        state: RenderState::Suspended(None),
        gui,
        widget: x,
        set_window,
        name: name.into()
    };
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.run_app(&mut app)
        .expect("couldnt run event loop");
    
}



/// Helper function that creates a Winit window and returns it (wrapped in an Arc for sharing between threads)
fn create_winit_window(event_loop: &ActiveEventLoop, name: &str) -> Arc<Window> {
    let attr = Window::default_attributes()
        .with_inner_size(LogicalSize::new(1044, 800))
        .with_resizable(true)
        .with_title(name);
    Arc::new(event_loop.create_window(attr).unwrap())
}

/// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions {
            surface_format: Some(surface.format),
            use_cpu: false,
            antialiasing_support: vello::AaSupport::all(),
            num_init_threads: NonZeroUsize::new(1),
        },
    )
    .expect("Couldn't create renderer")
}
