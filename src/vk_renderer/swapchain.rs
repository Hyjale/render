use std::sync::{Arc};

use ash::{vk::{self, Handle}};
use openxr as xr;

use crate::{vk_renderer::{
    framebuffer::Framebuffer
}};

const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
const VIEW_COUNT: u32 = 2;
const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

pub struct Swapchain {
    resolution: vk::Extent2D,
    handle: xr::Swapchain<xr::Vulkan>,
}

impl Swapchain {
    pub fn new(instance: &openxr::Instance,
               device: &ash::Device,
               session: &openxr::Session<xr::Vulkan>,
               system: openxr::SystemId,
    ) -> Arc<Swapchain> {
        unsafe {
            let views = instance
                .enumerate_view_configuration_views(system, VIEW_TYPE)
                .unwrap();

            let resolution = vk::Extent2D {
                width: views[0].recommended_image_rect_width,
                height: views[0].recommended_image_rect_height,
            };

            let handle = session
                .create_swapchain(&xr::SwapchainCreateInfo {
                    create_flags: xr::SwapchainCreateFlags::EMPTY,
                    usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                        | xr::SwapchainUsageFlags::SAMPLED,
                    format: COLOR_FORMAT.as_raw() as _,
                    sample_count: 1, // TODO no multi-sample
                    width: resolution.width,
                    height: resolution.height,
                    face_count: 1,
                    array_size: VIEW_COUNT,
                    mip_count: 1,
                })
                .unwrap();

            Arc::new(Swapchain {
                resolution,
                handle,
            })
        }
    }
}
