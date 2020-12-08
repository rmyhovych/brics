use super::{BindingHandle, BindingLayoutHandle};
use crate::{
    binding::buffer::{InstanceArrayBinding, InstanceArrayBindingLayout},
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

pub struct ObjectHandle {
    binding_layout: InstanceArrayBindingLayout,
    binding: InstanceArrayBinding,

    states: Vec<ObjectState>,
}

impl ObjectHandle {
    pub fn new(renderer: &Renderer, visibility: wgpu::ShaderStage, n_instances: u32) -> Self {
        let binding_layout =
            InstanceArrayBindingLayout::new::<ObjectState>(visibility, n_instances);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,

            states: (0..n_instances)
                .map(|_| ObjectState {
                    model: Matrix4::from_scale(1.0),
                    color: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                })
                .collect(),
        }
    }

    pub fn get_n_instances(&self) -> u32 {
        self.states.len() as u32
    }

    pub fn get_state(&self, instance: u32) -> &ObjectState {
        &self.states[instance as usize]
    }

    pub fn set_state(&mut self, state: ObjectState, instance: u32) {
        self.states[instance as usize] = state;
    }
}

impl BindingHandle<InstanceArrayBinding> for ObjectHandle {
    fn get_binding(&self) -> &InstanceArrayBinding {
        &self.binding
    }

    fn update(&mut self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.states, write_queue);
    }
}

impl BindingLayoutHandle<InstanceArrayBinding, InstanceArrayBindingLayout> for ObjectHandle {
    fn get_binding_layout(&self) -> &InstanceArrayBindingLayout {
        &self.binding_layout
    }
}
