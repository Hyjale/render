use openxr as xr;

pub struct Space {
    stage_space: openxr::Space,
    left_space: openxr::Space,
    right_space: openxr::Space,
}

impl Space {
    pub fn new(session: &openxr::Session<xr::Vulkan>,
               left_action: openxr::Action<xr::Posef>,
               right_action: openxr::Action<xr::Posef>
    ) -> Self {
        let stage_space = session
            .create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)
            .unwrap();

        let left_space = left_action
            .create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)
            .unwrap();
        let right_space = right_action
            .create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)
            .unwrap();

        Self {
            stage_space: stage_space,
            left_space: left_space,
            right_space: right_space,
        }
    }
}
