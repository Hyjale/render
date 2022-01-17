pub struct BindGroup {
    bind_group: wgpu::BindGroup
}

impl BindGroup {
    pub fn new(device: &wgpu::Device,
               bind_group_layout: &wgpu::BindGroupLayout,
               uniform_buf: &wgpu::Buffer
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding()
                },
            ],
            label: None
        });

        BindGroup {
            bind_group: bind_group
        }
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
