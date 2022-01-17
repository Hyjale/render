use crate::renderer::{
    vertex::Vertex
};

pub struct BufferData {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u16>,
    uniform_data: Vec<f32>,
}

impl BufferData {
    pub fn new(vertex_data: Vec<Vertex>, index_data: Vec<u16>, uniform_data: Vec<f32>) -> Self {
        BufferData {
            vertex_data,
            index_data,
            uniform_data
        }
    }

    pub fn get_vertex_data(&self) -> &Vec<Vertex> {
        &self.vertex_data
    }

    pub fn get_index_data(&self) -> &Vec<u16> {
        &self.index_data
    }

    pub fn get_uniform_data(&self) -> &Vec<f32> {
        &self.uniform_data
    }
}
