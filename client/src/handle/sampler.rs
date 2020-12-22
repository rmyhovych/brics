use super::{BindingHandle, BindingHandleLayout};
use crate::{
    binding::{
        sampler::{SamplerAddressMode, SamplerBinding, SamplerBindingLayout, SamplerFilterMode},
        Binding,
    },
    graphics::GraphicsManager,
};

/*--------------------------------------------------------------------------------------------------*/

pub struct SamplerHandleLayout {
    binding_layout: SamplerBindingLayout,
}

impl SamplerHandleLayout {
    pub fn new(
        visibility: wgpu::ShaderStage,
        address_mode: SamplerAddressMode,
        filter_mode: SamplerFilterMode,
        compare: Option<wgpu::CompareFunction>,
    ) -> Self {
        Self {
            binding_layout: SamplerBindingLayout::new(
                visibility,
                address_mode,
                filter_mode,
                compare,
            ),
        }
    }
}

impl BindingHandleLayout<SamplerBinding, SamplerBindingLayout, SamplerHandle>
    for SamplerHandleLayout
{
    fn get_binding_layout(&self) -> &SamplerBindingLayout {
        &self.binding_layout
    }

    fn create_handle(&self, graphics: &GraphicsManager) -> SamplerHandle {
        SamplerHandle::new(graphics.create_binding(&self.binding_layout))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct SamplerHandle {
    binding: SamplerBinding,
}

impl SamplerHandle {
    pub fn new(binding: SamplerBinding) -> Self {
        Self { binding }
    }
}

impl BindingHandle for SamplerHandle {
    fn get_binding(&self) -> &dyn Binding {
        &self.binding
    }

    fn update(&self, _: &wgpu::Queue) {}
}
