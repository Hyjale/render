use std::sync::{Arc};

use ash::{vk::{self}};

use crate::{VkHandle, impl_vk_handle};
use crate::vk_renderer::{
    device::Device
};

pub struct CommandPool {
    command_pool: ash::vk::CommandPool
}

impl CommandPool {
    pub fn new(device: Arc<Device>, queue_family_index: u32) -> Arc<CommandPool> {
        unsafe {
            let command_pool = device
                .borrow()
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::builder()
                        .queue_family_index(queue_family_index)
                        .flags(
                            vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                                | vk::CommandPoolCreateFlags::TRANSIENT,
                        ),
                    None,
                )
                .unwrap();

            Arc::new(CommandPool {
                command_pool: command_pool
            })
        }
    }
}

impl_vk_handle!(CommandPool, ash::vk::CommandPool, command_pool);
