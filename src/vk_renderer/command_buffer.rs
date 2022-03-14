use std::sync::{Arc};

use ash::{vk::{self}};

use crate::{VkHandle};
use crate::vk_renderer::{
    device::Device,
    pipeline::Pipeline,
    swapchain::Swapchain
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

    pub fn cmd_begin_render_pass(&self,
                                 cmd_buffer: ash::vk::CommandBuffer,
                                 render_pass: ash::vk::RenderPass,
                                 framebuffer: ash::vk::Framebuffer,
                                 extent: ash::vk::Extent2D
    ) {
        unsafe {
            self.device
                .borrow()
                .cmd_begin_render_pass(
                    cmd_buffer,
                    &vk::RenderPassBeginInfo::builder()
                        .render_pass(render_pass)
                        .framebuffer(framebuffer)
                        .render_area(vk::Rect2D {
                            offset: vk::Offset2D::default(),
                            extent: extent,
                        })
                        .clear_values(&[vk::ClearValue {
                            color: vk::ClearColorValue {
                                float32: [0.0, 0.0, 0.0, 1.0],
                            },
                        }]),
                    vk::SubpassContents::INLINE,
                )
        }
    }

    pub fn cmd_set_viewport(&self,
                            cmd_buffer: ash::vk::CommandBuffer,
                            viewports: &[ash::vk::Viewport]
    ) {
        unsafe {
            self.device
                .borrow()
                .cmd_set_viewport(cmd_buffer, 0, &viewports);
        }
    }

    pub fn cmd_set_scissor(&self,
                            cmd_buffer: ash::vk::CommandBuffer,
                            scissors: &[ash::vk::Rect2D]
    ) {
        unsafe {
            self.device
                .borrow()
                .cmd_set_scissor(cmd_buffer, 0, &scissors);
        }
    }

    pub fn cmd_bind_pipeline(&self,
                             cmd_buffer: ash::vk::CommandBuffer,
                             pipeline: &Arc<Pipeline>
    ) {
        unsafe {
            self.device
                .borrow()
                .cmd_bind_pipeline(cmd_buffer, pipeline.bind_point(), pipeline.vk_handle());
        }
    }

    pub fn cmd_end_render_pass(&self, cmd_buffer: ash::vk::CommandBuffer) {
        unsafe {
            self.device
                .borrow()
                .cmd_end_render_pass(cmd_buffer);
        }
    }

    pub fn cmd_draw(&self,
                    cmd_buffer: ash::vk::CommandBuffer,
                    vertex_count: u32,
                    instance_count: u32,
                    first_vertex: u32,
                    first_instance: u32
    ) {
        unsafe {
            self.device
                .borrow()
                .cmd_draw(cmd_buffer,
                          vertex_count,
                          instance_count,
                          first_vertex,
                          first_instance
                );
        }

    }
}
