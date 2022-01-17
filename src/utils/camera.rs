pub struct Camera {
    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32
}

impl Camera {
    pub fn new(fovy: f32, aspect: f32, near: f32, far: f32) -> Self{
        Self{
            fovy,
            aspect,
            near,
            far,
        }
    }

    pub fn create_view_projection_matrix(&self, view: cgmath::Matrix4<f32>) -> cgmath::Matrix4<f32> {
        let projection = cgmath::perspective(
            cgmath::Deg(self.fovy),
            self.aspect,
            self.near,
            self.far,
        );

        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
