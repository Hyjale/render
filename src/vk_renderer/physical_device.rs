use std::sync::{Arc};

use ash::{vk::{self, Handle}};

use crate::{VkHandle, impl_vk_handle};

pub struct PhysicalDevice {
    physical_device: ash::vk::PhysicalDevice
}

impl PhysicalDevice {
    pub fn new(instance: &openxr::Instance,
               vk_instance: &ash::Instance,
               system: openxr::SystemId
    ) -> Arc<PhysicalDevice> {
        let physical_device = vk::PhysicalDevice::from_raw(
            instance
                .vulkan_graphics_device(system, vk_instance.handle().as_raw() as _)
                .unwrap() as _,
        );

        Arc::new(PhysicalDevice {
            physical_device: physical_device,
        })
    }
}

impl_vk_handle!(PhysicalDevice, ash::vk::PhysicalDevice, physical_device);
