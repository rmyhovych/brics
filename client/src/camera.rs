use cgmath::{self, Matrix4, Point3, Quaternion, Rotation, Vector3};
use wgpu::{self, util::DeviceExt};

use crate::uniform::{Uniform, UniformDescriptor};

/*--------------------------------------------------------------------------------------------------*/

struct CameraUniform {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
}

impl Uniform for CameraUniform {}

impl CameraUniform {
    fn new(position: &Point3<f32>, direction: &Vector3<f32>, aspect_ratio: f32) -> CameraUniform {
        CameraUniform {
            projection: cgmath::perspective(cgmath::Deg(90.0), aspect_ratio, 0.01, 1000.0),
            view: cgmath::Matrix4::look_at_dir(*position, *direction, Vector3::unit_y()),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

struct Camera {
    uniform_buffer: wgpu::Buffer,

    position: Point3<f32>,
    direction: Vector3<f32>,
    aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        position: &Point3<f32>,
        direction: &Vector3<f32>,
        aspect_ratio: f32,
    ) -> Camera {
        Camera {
            uniform_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Camera Uniform Buffer"),
                size: CameraUniform::size(),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            }),
            position: *position,
            direction: *direction,
            aspect_ratio: aspect_ratio,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position += Vector3 { x, y, z }
    }

    pub fn rotate(&mut self, theta: f32, phi: f32) {
        let mut rotation = Quaternion::from_sv(phi, Vector3::unit_y().cross(self.direction));
        rotation = rotation * Quaternion::from_sv(theta, Vector3::unit_y());

        self.direction = rotation.rotate_vector(self.direction);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}

impl UniformDescriptor<CameraUniform> for Camera {
    fn get_uniform_buffer(&self) -> &wgpu::Buffer {
        &self.uniform_buffer
    }

    fn apply_on_renderpass<'a>(
        &'a self,
        renderpass: &wgpu::RenderPass<'a>,
        write_queue: &wgpu::Queue,
    ) {
        let uniform_data = CameraUniform::new(&self.position, &self.direction, self.aspect_ratio);
        self.update_uniform(write_queue, &uniform_data);
    }
}
