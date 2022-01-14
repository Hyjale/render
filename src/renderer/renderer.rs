use winit::window::Window;

use crate::renderer::{
    shader::Shader,
    pipeline::Pipeline
};

pub struct Renderer {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: Pipeline,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to create adapter");

        /* Logical Device */
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(), // for WebGL2 support, use downlevel_webgl2_defaults()
                },
                None,
            )
            .await
            .expect("Failed to create logical device");

        /* Shader Module */
        let shader = Shader::new(&device, include_str!("triangle.wgsl"));

        /* Render Pipeline */
        let pipeline = Pipeline::new(&device, &adapter, &surface, shader.get_shader());

        /* Surface Configuration */
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox // use Fifo for mobile
        };
        surface.configure(&device, &config);

        Renderer {
            instance: instance,
            adapter: adapter,
            device: device,
            queue: queue,
            pipeline: pipeline,
            surface: surface,
            config: config,
            size: size
        }
    }

    pub fn do_nothing(&self) {}
}
