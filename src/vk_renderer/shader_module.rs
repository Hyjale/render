use std::{io::Cursor};

use ash::{
    util::read_spv,
    vk::{self, Handle}
};

pub struct ShaderModule {
    vert: ash::vk::ShaderModule,
    frag: ash::vk::ShaderModule
}

impl ShaderModule {
    pub fn new(device: &ash::Device, vert_path: &[u8], frag_path: &[u8]) -> Self {
        let vert = read_spv(&mut Cursor::new(vert_path)).unwrap();
        let frag = read_spv(&mut Cursor::new(frag_path)).unwrap();

        unsafe {
            let vert = device
                .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&vert), None)
                .unwrap();
            let frag = device
                .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&frag), None)
                .unwrap();

            ShaderModule {
                vert: vert,
                frag: frag
            }
        }
    }
}
