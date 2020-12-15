use super::{BindingHandle, BindingHandleLayout};
use crate::{
    binding::{
        buffer::{UniformBinding, UniformBindingLayout},
        Binding,
    },
    renderer::Renderer,
};
use cgmath::{InnerSpace, Vector3};

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
#[repr(align(16))]
struct LightState {
    custom: Vector3<f32>,
    intensity: f32,
    color: Vector3<f32>,
}

impl LightState {
    fn new(custom: Vector3<f32>, intensity: f32, color: Vector3<f32>) -> Self {
        Self {
            custom,
            intensity,
            color,
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct LightHandleLayout {
    binding_layout: UniformBindingLayout,
}

impl LightHandleLayout {
    pub fn new(visibility: wgpu::ShaderStage) -> Self {
        Self {
            binding_layout: UniformBindingLayout::new::<LightState>(visibility),
        }
    }
}

impl BindingHandleLayout<UniformBinding, UniformBindingLayout, LightHandle> for LightHandleLayout {
    fn get_binding_layout(&self) -> &UniformBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, renderer: &Renderer) -> LightHandle {
        LightHandle::new(renderer.create_binding(&self.binding_layout))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct LightHandle {
    binding: UniformBinding,

    state: LightState,
}

impl LightHandle {
    pub fn new(binding: UniformBinding) -> Self {
        Self {
            binding,

            state: LightState::new(
                Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                1.2,
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            ),
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) -> &mut Self {
        self.state.custom = position;
        self
    }

    pub fn set_direction(&mut self, direction: Vector3<f32>) -> &mut Self {
        self.state.custom = direction.normalize();
        self
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        self.state.custom
    }

    pub fn set_color(&mut self, color: Vector3<f32>) -> &mut Self {
        self.state.color = color;
        self
    }
}

impl BindingHandle for LightHandle {
    fn get_binding(&self) -> &dyn Binding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.state, write_queue);
    }
}
