use crate::{
    binding::buffer::{UniformBinding, UniformBindingLayout},
    binding::{BindingHandle, BindingLayoutHandle},
    renderer::Renderer,
};

use cgmath::{Matrix4, Vector3};

use wgpu;

struct ObjectState {
    model: Matrix4<f32>,
    color: Vector3<f32>,
}

pub struct Object {
    binding_layout: UniformBindingLayout,
    binding: UniformBinding,

    state: ObjectState,
}

impl Object {
    pub fn new(
        renderer: &Renderer,
        position: &Vector3<f32>,
        scale: &Vector3<f32>,
        color: &Vector3<f32>,
    ) -> Self {
        let binding_layout = UniformBindingLayout::new::<ObjectState>(1, wgpu::ShaderStage::VERTEX);
        let binding = renderer.create_binding(&binding_layout);

        let model = Matrix4::from_translation(*position)
            * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

        Self {
            binding_layout,
            binding,
            state: ObjectState {
                model,
                color: *color,
            },
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.state.model = Matrix4::from_translation(Vector3 { x, y, z }) * self.state.model;
    }
}

/*--------------------------------------------------------------------------------------------------*/

impl BindingHandle<UniformBinding> for Object {
    fn get_binding(&self) -> &UniformBinding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.state, write_queue);
    }
}

impl BindingLayoutHandle<UniformBinding, UniformBindingLayout> for Object {
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }
}
