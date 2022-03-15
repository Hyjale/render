use std::sync::{Arc};

use ash::{vk::{self}};

use crate::{VkHandle};
use crate::vk_renderer::{
    command_buffer::CommandBuffer,
    command_pool::CommandPool,
    device::Device,
    fence::Fence,
    instance::Instance,
    physical_device::PhysicalDevice,
    pipeline::Pipeline,
    render_pass::RenderPass,
    swapchain::Swapchain,
};

pub struct Renderer {
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<Pipeline>
}

impl Renderer {
    pub fn new(instance: openxr::Instance, system: openxr::SystemId) -> Self {
        unsafe {
            // TODO VK Version asserts
            let target_version = vk::make_api_version(0, 1, 1, 0); // Vulkan 1.1 guarantees multiview support

            let entry = ash::Entry::load().unwrap();

            let app_info = vk::ApplicationInfo::builder()
                .application_version(0)
                .engine_version(0)
                .api_version(target_version);

            let vk_instance = Instance::new(&instance,
                                            &entry,
                                            system,
                                            app_info
            );

            let physical_device = PhysicalDevice::new(&instance,
                                                      &vk_instance.borrow(),
                                                      system,
            );

            let device = Device::new(&instance,
                                     &vk_instance.borrow(),
                                     &entry,
                                     physical_device.vk_handle(),
                                     system
            );

            let render_pass = RenderPass::new(&device);

            let pipeline = Pipeline::new(&device,
                                        render_pass.vk_handle()
            );

            let command_pool = CommandPool::new(&device, device.queue_family_index());

            let command_buffers = CommandBuffer::new(device.clone(), command_pool.vk_handle());

            let fences = Fence::new(&device);

            Renderer {
                device: device,
                render_pass: render_pass,
                pipeline: pipeline
            }
        }
    }

    // TODO - destroy resources
}
