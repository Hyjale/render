use wgpu::util::DeviceExt;

pub struct Buffer {
    buffer: wgpu::Buffer
}

impl Buffer {
    pub fn new(device: &wgpu::Device, desc: &wgpu::util::BufferInitDescriptor) -> Self {
        let buffer = device.create_buffer_init(desc);

        Buffer{
            buffer: buffer
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}
