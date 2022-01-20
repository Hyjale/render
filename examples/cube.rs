use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use renderer::{
    renderer::{
        buffer_data::BufferData,
        renderer::Renderer,
    },
    utils::{
        camera::Camera,
        geometry::Geometry,
    }
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    let size = window.inner_size();
    let (vertex_data, index_data) = Geometry::create_cube_data();
    let aspect_ratio = size.width as f32 / size.height as f32;
    let camera = Camera::new(45.0, aspect_ratio, 1.0, 10.0);
    let vp = camera.create_view_projection_matrix(
        cgmath::Matrix4::look_at_rh(
            cgmath::Point3::new(1.5f32, -5.0, 3.0),
            cgmath::Point3::new(0f32, 0.0, 0.0),
            cgmath::Vector3::unit_z()
        )
    );
    let vp_ref: &[f32; 16] = vp.as_ref();

    let index_count = index_data.len();
    let buffer_data = BufferData::new(vertex_data, index_data, vp_ref.to_vec());
    let mut renderer = Renderer::new(&window, &buffer_data).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.resize(size.width, size.height);
            },
            Event::RedrawRequested(_) => {
                renderer.draw(index_count);
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => {}
        }
    });
}
