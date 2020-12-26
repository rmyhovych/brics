use super::{BindingHandle, BindingHandleLayout};
use crate::{
    binding::{
        buffer::{UniformBinding, UniformBindingLayout},
        Binding,
    },
    graphics::GraphicsManager,
};

use cgmath::{InnerSpace, Matrix4, Quaternion, Rad, Vector3};

use wgpu;

/*--------------------------------------------------------------------------------------------------*/

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
struct ShapeState {
    model: Matrix4<f32>,
    color: Vector3<f32>,
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ShapeHandleLayout {
    binding_layout: UniformBindingLayout,
}

impl ShapeHandleLayout {
    pub fn new(visibility: wgpu::ShaderStage) -> Self {
        Self {
            binding_layout: UniformBindingLayout::new::<ShapeState>(visibility),
        }
    }
}

impl BindingHandleLayout<UniformBinding, UniformBindingLayout, ShapeHandle> for ShapeHandleLayout {
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, graphics: &GraphicsManager) -> ShapeHandle {
        ShapeHandle::new(graphics.create_binding(&self.binding_layout))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ShapeHandle {
    binding: UniformBinding,

    scale: Vector3<f32>,
    state: ShapeState,
}

impl ShapeHandle {
    pub fn new(binding: UniformBinding) -> Self {
        Self {
            binding,

            scale: Vector3::new(1.0, 1.0, 1.0),
            state: ShapeState {
                model: Matrix4::from_scale(1.0),
                color: Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f32>) {
        self.state.model =
            model * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
    }

    pub fn translate(&mut self, delta: Vector3<f32>) {
        self.state.model = Matrix4::from_translation(delta) * self.state.model;
    }

    pub fn rotate(&mut self, axis: Vector3<f32>, angle: f32) {
        self.state.model =
            self.state.model * Matrix4::from_axis_angle(axis.normalize(), Rad(angle));
    }

    pub fn set_color(&mut self, color: Vector3<f32>) {
        self.state.color = color;
    }

    pub fn rescale(&mut self, multiplier: Vector3<f32>) {
        self.scale.x = self.scale.x * multiplier.x;
        self.scale.y = self.scale.y * multiplier.y;
        self.scale.z = self.scale.z * multiplier.z;

        self.state.model = self.state.model
            * Matrix4::from_nonuniform_scale(multiplier.x, multiplier.y, multiplier.z);
    }
}

impl BindingHandle for ShapeHandle {
    fn get_binding(&self) -> &dyn Binding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.state, write_queue);
    }
}
