use openxr as xr;

pub struct Input {
    action_set: openxr::ActionSet,
    left_action: openxr::Action<xr::Posef>,
    right_action: openxr::Action<xr::Posef>,
}

impl Input {
    pub fn new(instance: &openxr::Instance,
    ) -> Self {
        let action_set = instance
            .create_action_set("input", "input pose information", 0)
            .unwrap();

        let left_action = action_set
            .create_action::<xr::Posef>("left_hand", "Left Hand Controller", &[])
            .unwrap();

        let right_action = action_set
            .create_action::<xr::Posef>("right_hand", "Right Hand Controller", &[])
            .unwrap();

        Self {
            action_set: action_set,
            left_action: left_action,
            right_action: right_action,
        }
    }
}
