use std::{io::Cursor};

use ash::{
    util::read_spv,
    vk::{self}
};

pub struct ShaderModule {
    pub vert: ash::vk::ShaderModule,
    pub frag: ash::vk::ShaderModule
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

    pub fn borrow_vert(&self) -> &ash::vk::ShaderModule {
        &self.vert
    }

    pub fn borrow_frag(&self) -> &ash::vk::ShaderModule {
        &self.frag
    }
}
