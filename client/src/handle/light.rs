use super::{BindingHandle, BindingLayoutHandle};
use crate::{
    binding::buffer::{UniformBinding, UniformBindingLayout},
    renderer::Renderer,
};
use cgmath::{InnerSpace, Vector3};

#[derive(Debug)]
#[repr(align(16))]
struct LightState {
    position: Vector3<f32>,
    __pad0: f32,
    direction: Vector3<f32>,
    __pad1: f32,
    color: Vector3<f32>,
}

impl LightState {
    fn new(position: Vector3<f32>, direction: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self {
            position,
            __pad0: 0.0,
            direction,
            __pad1: 0.0,
            color,
        }
    }
}

pub struct LightHandle {
    binding_layout: UniformBindingLayout,
    binding: UniformBinding,

    state: LightState,
}

impl LightHandle {
    pub fn new(renderer: &Renderer, binding: u32, visibility: wgpu::ShaderStage) -> Self {
        let binding_layout =
            UniformBindingLayout::new::<LightState>(binding, visibility);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,

            state: LightState::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            ),
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) -> &mut Self {
        self.state.position = position;
        self
    }

    pub fn set_direction(&mut self, direction: Vector3<f32>) -> &mut Self {
        self.state.direction = direction.normalize();
        self
    }

    pub fn set_color(&mut self, color: Vector3<f32>) -> &mut Self {
        self.state.color = color;
        self
    }
}

impl BindingHandle<UniformBinding> for LightHandle {
    fn get_binding(&self) -> &UniformBinding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.state, write_queue);
    }
}

impl BindingLayoutHandle<UniformBinding, UniformBindingLayout> for LightHandle {
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }
}
