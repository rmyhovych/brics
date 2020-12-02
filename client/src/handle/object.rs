use super::{BindingHandle, BindingLayoutHandle};
use crate::{
    binding::buffer::{
        InstanceArrayBinding, InstanceArrayBindingLayout, UniformBinding, UniformBindingLayout,
    },
    renderer::Renderer,
};

use cgmath::{Matrix4, Vector3};

use wgpu;

/*--------------------------------------------------------------------------------------------------*/

struct ObjectState {
    model: Matrix4<f32>,
    color: Vector3<f32>,
}

impl ObjectState {
    fn new() -> Self {
        Self {
            model: Matrix4::from_scale(1.0),
            color: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            // _padding: 0.0,
        }
    }
}

pub struct Object<'a> {
    state: &'a mut ObjectState,
}

impl Object<'_> {
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        (*self.state).model = Matrix4::from_translation(Vector3 { x, y, z }) * self.state.model;
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        (*self.state).color = Vector3 { x: r, y: g, z: b };
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectHandle {
    binding_layout: UniformBindingLayout,
    binding: UniformBinding,

    states: Vec<ObjectState>,
}

impl ObjectHandle {
    pub fn new(renderer: &Renderer) -> Self {
        let binding_layout = UniformBindingLayout::new::<ObjectState>(1, wgpu::ShaderStage::VERTEX);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,

            states: vec![ObjectState::new()],
        }
    }

    pub fn get_object(&mut self) -> Object {
        Object {
            state: &mut self.states[0],
        }
    }
}

impl BindingHandle<UniformBinding> for ObjectHandle {
    fn get_binding(&self) -> &UniformBinding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.states[0], write_queue);
    }
}

impl BindingLayoutHandle<UniformBinding, UniformBindingLayout> for ObjectHandle {
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct InstancedObjectHandle {
    binding_layout: InstanceArrayBindingLayout,
    binding: InstanceArrayBinding,

    states: Vec<ObjectState>,
}

impl InstancedObjectHandle {
    pub fn new(renderer: &Renderer, n_instances: u32) -> Self {
        let binding_layout = InstanceArrayBindingLayout::new::<ObjectState>(
            1,
            wgpu::ShaderStage::VERTEX,
            n_instances,
        );
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,

            states: (0..n_instances)
                .map(|_| ObjectState::new())
                .collect::<Vec<ObjectState>>(),
        }
    }

    pub fn get_object(&mut self, index: usize) -> Object {
        Object {
            state: &mut self.states[index],
        }
    }
}

impl BindingHandle<InstanceArrayBinding> for InstancedObjectHandle {
    fn get_binding(&self) -> &InstanceArrayBinding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.states, write_queue);
    }
}

impl BindingLayoutHandle<InstanceArrayBinding, InstanceArrayBindingLayout>
    for InstancedObjectHandle
{
    fn get_binding_layout(&self) -> &InstanceArrayBindingLayout {
        &self.binding_layout
    }
}
