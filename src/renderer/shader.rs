use std::borrow::Cow;

pub struct Shader {
    shader: wgpu::ShaderModule
}

impl Shader {
    pub fn new(device: &wgpu::Device, source: &str) -> Self {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
        });

        Shader {
            shader: shader
        }
    }

    pub fn get_shader(&self) -> &wgpu::ShaderModule {
        &self.shader
    }
}
