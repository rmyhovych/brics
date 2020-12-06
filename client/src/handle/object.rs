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

#[repr(align(16))]
pub struct Object {
    model: Matrix4<f32>,
    color: Vector3<f32>,
}

impl Object {
    fn new() -> Self {
        Self {
            model: Matrix4::from_scale(1.0),
            color: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.model = Matrix4::from_translation(Vector3 { x, y, z }) * self.model;

        self
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.color = Vector3 { x: r, y: g, z: b };

        self
    }

    pub fn rescale(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.model = self.model * Matrix4::from_nonuniform_scale(x, y, z);

        self
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectHandle {
    binding_layout: UniformBindingLayout,
    binding: UniformBinding,

    object: Object,
}

impl ObjectHandle {
    pub fn new(renderer: &Renderer, binding: u32, visibility: wgpu::ShaderStage) -> Self {
        let binding_layout = UniformBindingLayout::new::<Object>(binding, visibility);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,

            object: Object::new(),
        }
    }

    pub fn get_object(&mut self) -> &mut Object {
        &mut self.object
    }
}

impl BindingHandle<UniformBinding> for ObjectHandle {
    fn get_binding(&self) -> &UniformBinding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.object, write_queue);
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

    objects: Vec<Object>,
}

impl InstancedObjectHandle {
    pub fn new(
        renderer: &Renderer,
        binding: u32,
        visibility: wgpu::ShaderStage,
        n_instances: u32,
    ) -> Self {
        let binding_layout =
            InstanceArrayBindingLayout::new::<Object>(binding, visibility, n_instances);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,

            objects: (0..n_instances).map(|_| Object::new()).collect(),
        }
    }

    pub fn get_object(&mut self, index: usize) -> &mut Object {
        &mut self.objects[index]
    }

    pub fn get_n_instances(&self) -> u32 {
        self.objects.len() as u32
    }
}

impl BindingHandle<InstanceArrayBinding> for InstancedObjectHandle {
    fn get_binding(&self) -> &InstanceArrayBinding {
        &self.binding
    }

    fn update(&self, write_queue: &wgpu::Queue) {
        self.binding.update(&self.objects, write_queue);
    }
}

impl BindingLayoutHandle<InstanceArrayBinding, InstanceArrayBindingLayout>
    for InstancedObjectHandle
{
    fn get_binding_layout(&self) -> &InstanceArrayBindingLayout {
        &self.binding_layout
    }
}
