use cgmath::{self, InnerSpace, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3};
use wgpu::{self};

use crate::uniform::{Uniform, UniformDescriptor};

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
pub struct CameraUniform {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
}

impl Uniform for CameraUniform {}

impl CameraUniform {
    fn new(eye: &Point3<f32>, center: &Point3<f32>, aspect_ratio: f32) -> CameraUniform {
        CameraUniform {
            projection: cgmath::perspective(cgmath::Deg(60.0), aspect_ratio, 0.01, 100.0),
            view: cgmath::Matrix4::look_at(*eye, *center, Vector3::unit_y()),
        }
    }

    fn create_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: CameraUniform::size(),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Camera {
    uniform_buffer: wgpu::Buffer,

    eye: Point3<f32>,
    center: Point3<f32>,
    aspect_ratio: f32,
}

impl Camera {
    pub fn look_at(
        eye: &Point3<f32>,
        center: &Point3<f32>,
        aspect_ratio: f32,
        device: &wgpu::Device,
    ) -> Camera {
        Camera {
            uniform_buffer: CameraUniform::create_uniform_buffer(device),
            eye: *eye,
            center: *center,
            aspect_ratio: aspect_ratio,
        }
    }

    pub fn look_at_dir(
        eye: &Point3<f32>,
        direction: &Vector3<f32>,
        aspect_ratio: f32,
        device: &wgpu::Device,
    ) -> Camera {
        let center = eye + direction.normalize();
        Camera::look_at(eye, &center, aspect_ratio, device)
    }

    /*---------------------------------------------------------------------*/

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.eye += Vector3 { x, y, z };
    }

    pub fn set_center(&mut self, x: f32, y: f32, z: f32) {
        self.center = Point3 { x, y, z };
    }

    pub fn rotate_direction(&mut self, theta: f32, phi: f32) {
        let mut direction = self.get_direction();
        direction = self.rotate_vector(&direction, theta, phi);
        self.set_direction(&direction);
    }

    pub fn rotate_around_center(&mut self, theta: f32, phi: f32) {
        let from_center = self.eye - self.center;
        let rotated = self.rotate_vector(&from_center, theta, phi);
        self.eye = self.center + rotated;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    /*---------------------------------------------------------------------*/

    fn rotate_vector(&mut self, vector: &Vector3<f32>, theta: f32, phi: f32) -> Vector3<f32> {
        let size = vector.magnitude();
        let mut rotation =
            Quaternion::from_axis_angle(Vector3::unit_y().cross(*vector), cgmath::Rad(phi));
        rotation = rotation * Quaternion::from_axis_angle(Vector3::unit_y(), cgmath::Rad(theta));

        let rotated_v = rotation.rotate_vector(*vector);
        rotated_v.normalize_to(size)
    }

    fn get_direction(&self) -> Vector3<f32> {
        (self.center - self.eye).normalize()
    }

    fn set_direction(&mut self, direction: &Vector3<f32>) {
        self.center = self.eye + direction;
    }
}

impl UniformDescriptor<CameraUniform> for Camera {
    fn get_binding(&self) -> u32 {
        0
    }

    fn get_uniform_buffer(&self) -> &wgpu::Buffer {
        &self.uniform_buffer
    }

    fn apply_on_renderpass<'a>(&'a self, _: &mut wgpu::RenderPass<'a>, write_queue: &wgpu::Queue) {
        let uniform_data = CameraUniform::new(&self.eye, &self.center, self.aspect_ratio);
        self.write_uniform(write_queue, &uniform_data);
    }
}
