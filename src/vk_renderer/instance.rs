use std::sync::{Arc};

use ash::{vk::{self, Handle}};

pub struct Instance {
    instance: ash::Instance
}

impl Instance {
    pub fn new(instance: &openxr::Instance,
               entry: &ash::Entry,
               system: openxr::SystemId,
               app_info: ash::vk::ApplicationInfoBuilder
    ) -> Arc<Instance> {
        unsafe {
            let instance = {
                let instance = instance
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
                    vk::Instance::from_raw(instance as _),
                )
            };

            Arc::new(Instance {
                instance: instance
            })
        }
    }

    pub fn borrow(&self) -> &ash::Instance {
        &self.instance
    }
}
