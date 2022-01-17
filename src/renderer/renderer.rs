use winit::window::Window;

use crate::renderer::{
    bind_group::BindGroup,
    bind_group_layout::BindGroupLayout,
    buffer::Buffer,
    buffer_data::BufferData,
    pipeline::Pipeline,
    shader::Shader,
};

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: Pipeline,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    bind_group: BindGroup,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl Renderer {
    pub async fn new(window: &Window,
                     buffer_data: &BufferData

    ) -> Self {
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

        /* Vertex Buffer */
        let vertex_buffer = Buffer::new(
            &device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(buffer_data.get_vertex_data()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        /* Index Buffer */
        let index_buffer = Buffer::new(
            &device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(buffer_data.get_index_data()),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        /* Uniform Buffer */
        let uniform_buffer = Buffer::new(
            &device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(buffer_data.get_uniform_data()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        /* Shader Module */
        let shader = Shader::new(&device, include_str!("../../assets/shaders/cube.wgsl"));

        /* Bind Group Layout */
        let bind_group_layout = BindGroupLayout::new(&device);

        /* Bind Group */
        let bind_group = BindGroup::new(&device,
                                        bind_group_layout.get_bind_group_layout(),
                                        uniform_buffer.get_buffer(),
        );

        /* Render Pipeline */
        let pipeline = Pipeline::new(&device,
                                     &adapter,
                                     &surface,
                                     bind_group_layout.get_bind_group_layout(),
                                     shader.get_shader());

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
            bind_group: bind_group,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }

    pub fn draw(&self, index_count: usize) {
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
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true
                    },
                }],
                depth_stencil_attachment: None,
            });

            let pipeline = self.pipeline.get_pipeline();
            let bind_group = self.bind_group.get_bind_group();
            let vertex_buffer = self.vertex_buffer.get_buffer();
            let index_buffer = self.index_buffer.get_buffer();
            rpass.set_pipeline(pipeline);
            rpass.set_bind_group(0, bind_group, &[]);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rpass.draw_indexed(0..index_count as u32, 0, 0..1);
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
