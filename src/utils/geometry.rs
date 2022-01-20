use crate::renderer::{
    vertex::Vertex,
};

fn vertex(pos: [i8; 3]) -> Vertex {
    Vertex {
        pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0]
    }
}

pub struct Geometry;

impl Geometry {
    pub fn create_cube_data() -> (Vec<Vertex>, Vec<u16>) {
        let vertex_data = [
            vertex([-1, -1, 1]),
            vertex([1, -1, 1]),
            vertex([1, 1, 1]),
            vertex([-1, 1, 1]),
            vertex([-1, 1, -1]),
            vertex([1, 1, -1]),
            vertex([1, -1, -1]),
            vertex([-1, -1, -1]),
            vertex([1, -1, -1]),
            vertex([1, 1, -1]),
            vertex([1, 1, 1]),
            vertex([1, -1, 1]),
            vertex([-1, -1, 1]),
            vertex([-1, 1, 1]),
            vertex([-1, 1, -1]),
            vertex([-1, -1, -1]),
            vertex([1, 1, -1]),
            vertex([-1, 1, -1]),
            vertex([-1, 1, 1]),
            vertex([1, 1, 1]),
            vertex([1, -1, 1]),
            vertex([-1, -1, 1]),
            vertex([-1, -1, -1]),
            vertex([1, -1, -1]),
        ];

        let index_data: &[u16] = &[
            0, 1, 2, 2, 3, 0,
            4, 5, 6, 6, 7, 4,
            8, 9, 10, 10, 11, 8,
            12, 13, 14, 14, 15, 12,
            16, 17, 18, 18, 19, 16,
            20, 21, 22, 22, 23, 20,
        ];

        (vertex_data.to_vec(), index_data.to_vec())
    }
}
