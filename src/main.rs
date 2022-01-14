mod renderer;

use winit::{
    event_loop::{EventLoop},
    window::Window,
};

use crate::renderer::{
   renderer::Renderer
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    let renderer = Renderer::new(&window).await;
    renderer.do_nothing();
}
