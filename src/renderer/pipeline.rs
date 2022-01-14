pub struct Pipeline {
    pipeline: wgpu::RenderPipeline
}

impl Pipeline {
    pub fn new(device: &wgpu::Device,
               adapter: &wgpu::Adapter,
               surface: &wgpu::Surface,
               shader: &wgpu::ShaderModule,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[swapchain_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        });

        Pipeline {
            pipeline: pipeline
        }
    }

    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
