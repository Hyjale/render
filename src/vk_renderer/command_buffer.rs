use std::sync::{Arc};

use ash::{vk::{self}};

use crate::vk_renderer::{
    device::Device
};

const PIPELINE_DEPTH: u32 = 2;

pub struct CommandBuffer {
    device: Arc<Device>,
    command_buffer: Vec<ash::vk::CommandBuffer>
}

impl CommandBuffer {
    pub fn new(device: Arc<Device>, command_pool: ash::vk::CommandPool) -> Arc<CommandBuffer> {
        unsafe {
            let command_buffers = device
                .borrow()
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pool)
                        .command_buffer_count(PIPELINE_DEPTH),
                )
                .unwrap();

            Arc::new(CommandBuffer {
                device: device,
                command_buffer: command_buffers
            })
        }
    }
}
