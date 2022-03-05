use std::sync::{Arc};

use ash::{vk::{self, Handle}};

use crate::{VkHandle, impl_vk_handle};

pub struct Device {
    device: ash::Device,
    queue: ash::vk::Queue,
}

impl Device {
    pub fn new(instance: openxr::Instance, system: openxr::SystemId) -> Arc<Device> {
        unsafe {
            // TODO VK Version asserts
            let target_version = vk::make_api_version(0, 1, 1, 0); // Vulkan 1.1 guarantees multiview support

            let entry = ash::Entry::load().unwrap();

            let app_info = vk::ApplicationInfo::builder()
                .application_version(0)
                .engine_version(0)
                .api_version(target_version);

            let vk_instance = {
                let vk_instance = instance
                    .create_vulkan_instance(
                        system,
                        std::mem::transmute(entry.static_fn().get_instance_proc_addr),
                        &vk::InstanceCreateInfo::builder().application_info(&app_info) as *const _
                            as *const _,
                    )
                    .expect("OpenXR error creating Vulkan instance")
                    .map_err(vk::Result::from_raw)
                    .expect("Vulkan error creating Vulkan instance");
                ash::Instance::load(
                    entry.static_fn(),
                    vk::Instance::from_raw(vk_instance as _),
                )
            };

            let vk_physical_device = vk::PhysicalDevice::from_raw(
                instance
                    .vulkan_graphics_device(system, vk_instance.handle().as_raw() as _)
                    .unwrap() as _,
            );

            let queue_family_index = vk_instance
                .get_physical_device_queue_family_properties(vk_physical_device)
                .into_iter()
                .enumerate()
                .find_map(|(queue_family_index, info)| {
                    if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        Some(queue_family_index as u32)
                    } else {
                        None
                    }
                })
                .unwrap();

            let device = {
                let device_queue_create_info = [vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_family_index)
                    .queue_priorities(&[1.0])
                    .build()];

                let mut multiview_features = vk::PhysicalDeviceMultiviewFeatures {
                    multiview: vk::TRUE,
                    ..Default::default()
                };

                let device_create_info = vk::DeviceCreateInfo::builder()
                    .queue_create_infos(&device_queue_create_info)
                    .push_next(&mut multiview_features);

                let device = instance
                    .create_vulkan_device(
                        system,
                        std::mem::transmute(entry.static_fn().get_instance_proc_addr),
                        vk_physical_device.as_raw() as _,
                        &device_create_info as *const _ as *const _,
                    )
                    .expect("OpenXR error creating Vulkan device")
                    .map_err(vk::Result::from_raw)
                    .expect("Vulkan error creating Vulkan device");

                ash::Device::load(vk_instance.fp_v1_0(), vk::Device::from_raw(device as _))
            };

            let queue = device.get_device_queue(queue_family_index, 0);

            Arc::new(Device {
                device: device,
                queue: queue
            })
        }
    }

    pub fn borrow(&self) -> &ash::Device {
        &self.device
    }
}
