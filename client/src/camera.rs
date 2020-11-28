use cgmath::{self, InnerSpace, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3};
use wgpu::{self};

use crate::binding::{
    buffer::{UniformBinding, UniformBindingLayout},
    BindingLayout,
};

use crate::resource;

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
pub struct CameraUniform {
    pv: Matrix4<f32>,
}

impl CameraUniform {
    fn new(eye: &Point3<f32>, center: &Point3<f32>, aspect_ratio: f32) -> CameraUniform {
        CameraUniform {
            pv: cgmath::perspective(cgmath::Deg(60.0), aspect_ratio, 0.01, 1000.0)
                * cgmath::Matrix4::look_at(*eye, *center, Vector3::unit_y()),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Camera {
    uniform_binding_layout: UniformBindingLayout,
    uniform_binding: UniformBinding,

    eye: Point3<f32>,
    center: Point3<f32>,
    aspect_ratio: f32,
}

impl Camera {
    pub fn look_at(
        device: &wgpu::Device,

        eye: &Point3<f32>,
        center: &Point3<f32>,
        aspect_ratio: f32,
    ) -> Camera {
        let uniform_binding_layout: UniformBindingLayout =
            UniformBindingLayout::new::<CameraUniform>(0, wgpu::ShaderStage::VERTEX);
        let uniform_binding = uniform_binding_layout.create_binding(device);

        Camera {
            uniform_binding_layout,
            uniform_binding,

            eye: *eye,
            center: *center,
            aspect_ratio: aspect_ratio,
        }
    }

    pub fn look_at_dir(
        device: &wgpu::Device,

        eye: &Point3<f32>,
        direction: &Vector3<f32>,
        aspect_ratio: f32,
    ) -> Camera {
        let center = eye + direction.normalize();
        Camera::look_at(device, eye, &center, aspect_ratio)
    }

    /*---------------------------------------------------------------------*/

    pub fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.uniform_binding_layout
    }

    pub fn get_binding(&self) -> &UniformBinding {
        &self.uniform_binding
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
        let mut rotation = Quaternion::from_axis_angle(
            Vector3::unit_y().cross(*vector).normalize(),
            cgmath::Rad(phi),
        );
        rotation = rotation * Quaternion::from_axis_angle(Vector3::unit_y(), cgmath::Rad(theta));

        let rotated_v = rotation.normalize().rotate_vector(*vector);
        rotated_v.normalize_to(size)
    }

    fn get_direction(&self) -> Vector3<f32> {
        (self.center - self.eye).normalize()
    }

    fn set_direction(&mut self, direction: &Vector3<f32>) {
        self.center = self.eye + direction;
    }
}

impl resource::DynamicResource for Camera {
    fn update(&self, write_queue: &wgpu::Queue) {
        let uniform_data = CameraUniform::new(&self.eye, &self.center, self.aspect_ratio);
        self.uniform_binding.update(&uniform_data, write_queue);
    }
}
