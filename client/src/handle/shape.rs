use super::{BindingHandle, BindingHandleLayout};
use crate::{
    binding::{
        buffer::{UniformBindingLayout, UniformBinding},
        Binding,
    },
    renderer::Renderer,
};

use cgmath::{Matrix4, Vector3};

use wgpu;

/*--------------------------------------------------------------------------------------------------*/

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct ShapeState {
    pub model: Matrix4<f32>,
    pub color: Vector3<f32>,
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

impl BindingHandleLayout<UniformBinding, UniformBindingLayout, ShapeHandle>
    for ShapeHandleLayout
{
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, renderer: &Renderer) -> ShapeHandle {
        ShapeHandle::new(
            renderer.create_binding(&self.binding_layout),
        )
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ShapeHandle {
    binding: UniformBinding,

    state: ShapeState,
}

impl ShapeHandle {
    pub fn new(binding: UniformBinding) -> Self {
        Self {
            binding,

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

    pub fn get_state(&self) -> &ShapeState {
        &self.state
    }

    pub fn set_state(&mut self, state: ShapeState) {
        self.state = state;
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
