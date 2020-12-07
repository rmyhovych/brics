use super::{BindingHandle, BindingLayoutHandle};
use crate::{
    binding::texture::{TextureBinding, TextureBindingLayout},
    renderer::Renderer,
};

/*--------------------------------------------------------------------------------------------------*/

pub struct TextureHandle {
    binding_layout: TextureBindingLayout,
    binding: TextureBinding,
}

impl TextureHandle {
    pub fn new(
        renderer: &Renderer,
        visibility: wgpu::ShaderStage,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> Self {
        let binding_layout = TextureBindingLayout::new_sampled_output(visibility, size, format);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,
        }
    }
}

impl BindingHandle<TextureBinding> for TextureHandle {
    fn get_binding(&self) -> &TextureBinding {
        &self.binding
    }

    fn update(&self, _: &wgpu::Queue) {
        // TODO
    }
}

impl BindingLayoutHandle<TextureBinding, TextureBindingLayout> for TextureHandle {
    fn get_binding_layout(&self) -> &TextureBindingLayout {
        &self.binding_layout
    }
}
