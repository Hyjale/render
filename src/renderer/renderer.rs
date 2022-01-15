use winit::window::Window;

use crate::renderer::{
    shader::Shader,
    pipeline::Pipeline
};

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: Pipeline,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
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
            device: device,
            queue: queue,
            pipeline: pipeline,
            surface: surface,
            config: config,
        }
    }

    pub fn draw(&self) {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to get next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: None});
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true
                    },
                }],
                depth_stencil_attachment: None,
            });

            let pipeline = self.pipeline.get_pipeline();
            rpass.set_pipeline(pipeline);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);

    }
}
