use super::{BindingHandle, BindingHandleLayout};
use crate::{
    binding::{
        texture::{TextureBinding, TextureBindingLayout},
        Binding,
    },
    renderer::Renderer,
};

/*--------------------------------------------------------------------------------------------------*/

pub struct TextureHandleLayout {
    binding_layout: TextureBindingLayout,
}

impl TextureHandleLayout {
    pub fn new(
        visibility: wgpu::ShaderStage,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            binding_layout: TextureBindingLayout::new_sampled_output(visibility, size, format),
        }
    }
}

impl BindingHandleLayout<TextureBinding, TextureBindingLayout, TextureHandle>
    for TextureHandleLayout
{
    fn get_binding_layout(&self) -> &TextureBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, renderer: &Renderer) -> TextureHandle {
        TextureHandle::new(renderer.create_binding(&self.binding_layout))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct TextureHandle {
    binding: TextureBinding,
}

impl TextureHandle {
    pub fn new(binding: TextureBinding) -> Self {
        Self { binding }
    }

    pub fn create_texture_view(&self) -> wgpu::TextureView {
        self.binding.create_texture_view()
    }
}

impl BindingHandle for TextureHandle {
    fn get_binding(&self) -> &dyn Binding {
        &self.binding
    }

    fn update(&self, _: &wgpu::Queue) {
        // TODO
    }
}
