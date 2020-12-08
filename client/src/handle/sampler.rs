use super::{BindingHandle, BindingLayoutHandle};
use crate::{
    binding::sampler::{
        SamplerAddressMode, SamplerBinding, SamplerBindingLayout, SamplerFilterMode,
    },
    renderer::Renderer,
};

/*--------------------------------------------------------------------------------------------------*/

pub struct SamplerHandle {
    binding_layout: SamplerBindingLayout,
    binding: SamplerBinding,
}

impl SamplerHandle {
    pub fn new(
        renderer: &Renderer,
        visibility: wgpu::ShaderStage,
        address_mode: SamplerAddressMode,
        filter_mode: SamplerFilterMode,
        compare: Option<wgpu::CompareFunction>,
    ) -> Self {
        let binding_layout =
            SamplerBindingLayout::new(visibility, address_mode, filter_mode, compare);
        let binding = renderer.create_binding(&binding_layout);

        Self {
            binding_layout,
            binding,
        }
    }
}

impl BindingHandle<SamplerBinding> for SamplerHandle {
    fn get_binding(&self) -> &SamplerBinding {
        &self.binding
    }

    fn update(&mut self, _: &wgpu::Queue) {}
}

impl BindingLayoutHandle<SamplerBinding, SamplerBindingLayout> for SamplerHandle {
    fn get_binding_layout(&self) -> &SamplerBindingLayout {
        &self.binding_layout
    }
}
