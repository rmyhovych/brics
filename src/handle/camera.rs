use cgmath::{self, InnerSpace, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3};
use wgpu::{self};

use super::{BindingHandle, BindingHandleLayout};
use crate::{
    binding::{
        buffer::{UniformBinding, UniformBindingLayout},
        Binding,
    },
    graphics::GraphicsManager,
};

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
#[repr(align(16))]
pub struct CameraState {
    pv: Matrix4<f32>,
    position: Point3<f32>,
}

impl CameraState {
    fn new(projection: &Matrix4<f32>, eye: &Point3<f32>, center: &Point3<f32>) -> CameraState {
        CameraState {
            pv: projection * cgmath::Matrix4::look_at(*eye, *center, Vector3::unit_y()),
            position: *eye,
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct CameraHandleLayout {
    binding_layout: UniformBindingLayout,
}

impl CameraHandleLayout {
    pub fn new(visibility: wgpu::ShaderStage) -> Self {
        Self {
            binding_layout: UniformBindingLayout::new::<CameraState>(visibility),
        }
    }
}

impl BindingHandleLayout<UniformBinding, UniformBindingLayout, CameraHandle>
    for CameraHandleLayout
{
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, graphics: &GraphicsManager) -> CameraHandle {
        CameraHandle::new(graphics.create_binding(&self.binding_layout))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct CameraHandle {
    binding: UniformBinding,

    projection: Matrix4<f32>,

    eye: Point3<f32>,
    center: Point3<f32>,
}

impl CameraHandle {
    pub fn new(binding: UniformBinding) -> Self {
        Self {
            binding,

            projection: Matrix4::from_scale(1.0),

            eye: Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            center: Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn look_at_dir(&mut self, eye: Point3<f32>, direction: Vector3<f32>) -> &mut Self {
        let center = eye + direction.normalize();
        self.look_at(eye, center)
    }

    pub fn look_at(&mut self, eye: Point3<f32>, center: Point3<f32>) -> &mut Self {
        self.eye = eye;
        self.center = center;

        self
    }

    pub fn set_perspective(&mut self, angle: f32, aspect_ratio: f32) -> &mut Self {
        self.projection = cgmath::perspective(cgmath::Deg(angle), aspect_ratio, 0.01, 1000.0);

        self
    }

    pub fn set_ortho(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> &mut Self {
        self.projection = cgmath::ortho(left, right, bottom, top, near, far);

        self
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        let delta = Vector3 { x, y, z };
        self.eye += delta;
        self.center += delta;

        self
    }

    pub fn set_center(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        let dir = self.center - self.eye;
        self.center = Point3::new(x, y, z);
        self.eye = self.center + dir;

        self
    }

    pub fn rotate_direction(&mut self, theta: f32, phi: f32) -> &mut Self {
        let mut direction = self.get_direction();
        direction = self.rotate_vector(&direction, theta, phi);
        self.set_direction(&direction);

        self
    }

    pub fn rotate_around_center(&mut self, theta: f32, phi: f32) -> &mut Self {
        let from_center = self.eye - self.center;
        let rotated = self.rotate_vector(&from_center, theta, phi);
        self.eye = self.center + rotated;

        self
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        (self.center - self.eye).normalize()
    }

    pub fn get_center(&self) -> Point3<f32> {
        self.center
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

impl BindingHandle for CameraHandle {
    fn get_binding(&self) -> &dyn Binding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        let uniform_data = CameraState::new(&self.projection, &self.eye, &self.center);
        self.binding.update(&uniform_data, write_queue);
    }
}
