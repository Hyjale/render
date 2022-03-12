use std::sync::{Arc};

use ash::{vk::{self, Handle}};
use openxr as xr;

pub struct Session {
    session: openxr::Session<xr::Vulkan>,
    frame_wait: openxr::FrameWaiter,
    frame_stream: openxr::FrameStream<xr::Vulkan>,
}

impl Session {
    pub fn new(instance: &openxr::Instance,
               vk_instance: &ash::Instance,
               system: openxr::SystemId,
               physical_device: &ash::vk::PhysicalDevice,
               device: ash::Device,
               queue_family_index: u32
    ) -> Self {
        unsafe {
            let (session, frame_wait, frame_stream) = instance
                .create_session::<xr::Vulkan>(
                    system,
                    &xr::vulkan::SessionCreateInfo {
                        instance: vk_instance.handle().as_raw() as _,
                        physical_device: physical_device.as_raw() as _,
                        device: device.handle().as_raw() as _,
                        queue_family_index,
                        queue_index: 0,
                    },
                )
                .unwrap();

            Session {
                session: session,
                frame_wait: frame_wait,
                frame_stream: frame_stream
            }
        }
    }
}
