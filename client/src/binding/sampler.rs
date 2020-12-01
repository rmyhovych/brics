use super::{Binding, BindingLayout};

pub struct SamplerAddressMode {
    pub u: wgpu::AddressMode,
    pub v: wgpu::AddressMode,
    pub w: wgpu::AddressMode,
}

pub struct SamplerFilterMode {
    pub mag: wgpu::FilterMode,
    pub min: wgpu::FilterMode,
    pub mipmap: wgpu::FilterMode,
}

pub struct SamplerBindingLayout {
    binding: u32,
    visibility: wgpu::ShaderStage,

    address_mode: SamplerAddressMode,
    filter_mode: SamplerFilterMode,
    compare: Option<wgpu::CompareFunction>,
}

impl SamplerBindingLayout {
    pub fn new(
        binding: u32,
        visibility: wgpu::ShaderStage,
        address_mode: SamplerAddressMode,
        filter_mode: SamplerFilterMode,
        compare: Option<wgpu::CompareFunction>,
    ) -> Self {
        Self {
            binding,
            visibility,

            address_mode,
            filter_mode,
            compare,
        }
    }
}

impl BindingLayout<SamplerBinding> for SamplerBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility,
            ty: wgpu::BindingType::Sampler {
                comparison: self.compare != None,
            },
            count: None,
        }
    }

    fn create_binding(&self, device: &wgpu::Device) -> SamplerBinding {
        SamplerBinding {
            sampler: device.create_sampler(&wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: self.address_mode.u,
                address_mode_v: self.address_mode.v,
                address_mode_w: self.address_mode.w,

                mag_filter: self.filter_mode.mag,
                min_filter: self.filter_mode.min,
                mipmap_filter: self.filter_mode.mipmap,
                lod_min_clamp: 0.0,
                lod_max_clamp: std::f32::MAX,
                compare: self.compare,
                anisotropy_clamp: None,
            }),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct SamplerBinding {
    sampler: wgpu::Sampler,
}

impl Binding for SamplerBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Sampler(&self.sampler)
    }
}
