use openxr as xr;

fn main() {
    let entry = xr::Entry::load()
        .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature");

    let extensions = entry.enumerate_extensions().unwrap();
    println!("Extensions: {:#?}", extensions);

    let instance = entry
        .create_instance(
            &xr::ApplicationInfo {
                application_name: "openxr",
                ..Default::default()
            },
            &xr::ExtensionSet::default(),
            &[],
        ).unwrap();
    let instance_props = instance.properties().unwrap();

    let system = instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();
}
