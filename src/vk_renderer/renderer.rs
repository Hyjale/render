use std::sync::{Arc};

use crate::{VkHandle};
use crate::vk_renderer::{
    device::Device,
    pipeline::Pipeline,
    render_pass::RenderPass,
};

pub struct Renderer {
    device: Arc<Device>
}

impl Renderer {
    pub fn new(instance: openxr::Instance, system: openxr::SystemId) -> Self {
        let device = Device::new(instance, system);

        let render_pass = RenderPass::new(&device.borrow());

        let pipeline = Pipeline::new(&device.borrow(),
                                     render_pass.vk_handle()
        );

        Renderer {
            device: device
        }
    }
}
