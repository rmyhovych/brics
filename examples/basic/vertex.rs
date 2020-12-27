use cgmath::{Point3, Vector3};
use brics::pipeline::Vertex;

pub struct VertexBasic {
    _position: Point3<f32>,
    _normal: Vector3<f32>,
}

impl VertexBasic {
    pub fn new(position: Point3<f32>, normal: Vector3<f32>) -> Self {
        Self {
            _position: position,
            _normal: normal,
        }
    }
}

impl Vertex for VertexBasic {
    fn get_attribute_formats() -> Vec<wgpu::VertexFormat> {
        vec![wgpu::VertexFormat::Float3, wgpu::VertexFormat::Float3]
    }
}
