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
pub struct ObjectState {
    pub model: Matrix4<f32>,
    pub color: Vector3<f32>,
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectHandleLayout {
    binding_layout: UniformBindingLayout,
}

impl ObjectHandleLayout {
    pub fn new(visibility: wgpu::ShaderStage) -> Self {
        Self {
            binding_layout: UniformBindingLayout::new::<ObjectState>(visibility),
        }
    }
}

impl BindingHandleLayout<UniformBinding, UniformBindingLayout, ObjectHandle>
    for ObjectHandleLayout
{
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, renderer: &Renderer) -> ObjectHandle {
        ObjectHandle::new(
            renderer.create_binding(&self.binding_layout),
        )
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectHandle {
    binding: UniformBinding,

    state: ObjectState,
}

impl ObjectHandle {
    pub fn new(binding: UniformBinding) -> Self {
        Self {
            binding,

            state: ObjectState {
                    model: Matrix4::from_scale(1.0),
                    color: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
        }
    }

    pub fn get_state(&self) -> &ObjectState {
        &self.state
    }

    pub fn set_state(&mut self, state: ObjectState) {
        self.state = state;
    }
}

impl BindingHandle for ObjectHandle {
    fn get_binding(&self) -> &dyn Binding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.state, write_queue);
    }
}
