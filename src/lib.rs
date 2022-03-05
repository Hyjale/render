pub mod renderer;
pub mod utils;
pub mod vk_renderer;

pub trait VkHandle<T> {
    fn vk_handle(&self) -> T;
}

#[macro_export]
macro_rules! impl_vk_handle {
    ($struct_name:ident, $type_name:path, $value:ident) => {
        impl VkHandle<$type_name> for $struct_name {
            fn vk_handle(&self) -> $type_name {
                self.$value
            }
        }

        impl<'a> VkHandle<$type_name> for &'a $struct_name {
            fn vk_handle(&self) -> $type_name {
                self.$value
            }
        }

        impl VkHandle<$type_name> for Arc<$struct_name> {
            fn vk_handle(&self) -> $type_name {
                self.$value
            }
        }

        impl<'a> VkHandle<$type_name> for &'a Arc<$struct_name> {
            fn vk_handle(&self) -> $type_name {
                self.$value
            }
        }
    };
}
