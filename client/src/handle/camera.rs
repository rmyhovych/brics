use cgmath::{self, InnerSpace, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3};
use wgpu::{self};

use super::{BindingHandle, BindingLayoutHandle};
use crate::{
    binding::{
        buffer::{UniformBinding, UniformBindingLayout},
        BindingLayout,
    },
    renderer::Renderer,
};

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
#[repr(align(16))]
pub struct CameraUniform {
    pv: Matrix4<f32>,
    position: Point3<f32>,
}

impl CameraUniform {
    fn new(
        projection: &Matrix4<f32>,
        eye: &Point3<f32>,
        center: &Point3<f32>,
        aspect_ratio: f32,
    ) -> CameraUniform {
        CameraUniform {
            pv: projection * cgmath::Matrix4::look_at(*eye, *center, Vector3::unit_y()),
            position: *eye,
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct CameraHandle {
    uniform_binding_layout: UniformBindingLayout,
    uniform_binding: UniformBinding,

    projection: Matrix4<f32>,

    eye: Point3<f32>,
    center: Point3<f32>,
    aspect_ratio: f32,
}

impl CameraHandle {
    pub fn look_at(
        renderer: &Renderer,

        eye: &Point3<f32>,
        center: &Point3<f32>,
        aspect_ratio: f32,
    ) -> Self {
        let uniform_binding_layout: UniformBindingLayout =
            UniformBindingLayout::new::<CameraUniform>(0, wgpu::ShaderStage::VERTEX);
        let uniform_binding = renderer.create_binding(&uniform_binding_layout);

        Self {
            uniform_binding_layout,
            uniform_binding,

            projection: cgmath::perspective(cgmath::Deg(60.0), aspect_ratio, 0.01, 1000.0),

            eye: *eye,
            center: *center,
            aspect_ratio: aspect_ratio,
        }
    }

    pub fn look_at_dir(
        renderer: &Renderer,

        eye: &Point3<f32>,
        direction: &Vector3<f32>,
        aspect_ratio: f32,
    ) -> Self {
        let center = eye + direction.normalize();
        Self::look_at(renderer, eye, &center, aspect_ratio)
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let delta = Vector3 { x, y, z };
        self.eye += delta;
        self.center += delta;
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

    pub fn get_direction(&self) -> Vector3<f32> {
        (self.center - self.eye).normalize()
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

    fn set_direction(&mut self, direction: &Vector3<f32>) {
        self.center = self.eye + direction;
    }
}

/*----------------------------------------------------------------------------------*/

impl BindingHandle<UniformBinding> for CameraHandle {
    fn get_binding(&self) -> &UniformBinding {
        &self.uniform_binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        let uniform_data =
            CameraUniform::new(&self.projection, &self.eye, &self.center, self.aspect_ratio);
        self.uniform_binding.update(&uniform_data, write_queue);
    }
}

impl BindingLayoutHandle<UniformBinding, UniformBindingLayout> for CameraHandle {
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.uniform_binding_layout
    }
}
