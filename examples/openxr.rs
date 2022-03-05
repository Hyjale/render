use openxr as xr;

use xr_renderer::{
    vk_renderer::{
        device::Device,
        pipeline::Pipeline,
        render_pass::RenderPass,
        shader_module::ShaderModule,
    }
};

#[cfg_attr(target_os = "android", ndk_glue::main)]
fn main() {
    let entry = xr::Entry::load()
        .expect("Couldn't find the OpenXR loader; try enabling the \"static\" feature");

    #[cfg(target_os = "android")]
    entry.initialize_android_loader().unwrap();

    let extensions = entry.enumerate_extensions().unwrap();
    println!("Extensions: {:#?}", extensions);

    assert!(extensions.khr_vulkan_enable || extensions.khr_vulkan_enable2);

    let mut enabled_extensions = xr::ExtensionSet::default();
    if extensions.khr_vulkan_enable2 {
        enabled_extensions.khr_vulkan_enable2 = true;
    } else {
        enabled_extensions.khr_vulkan_enable = true;
    }
    #[cfg(target_os = "android")]
    {
        enabled_extensions.khr_android_create_instance = true;
    }

    let instance = entry
        .create_instance(
            &xr::ApplicationInfo {
                application_name: "openxr",
                ..Default::default()
            },
            &enabled_extensions,
            &[],
        ).unwrap();
    let instance_props = instance.properties().unwrap();
    println!(
        "Loaded OpenXR runtime: {} {}",
        instance_props.runtime_name, instance_props.runtime_version
    );

    let system = instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();

    // TODO Destroy shader modules and everything else
}
