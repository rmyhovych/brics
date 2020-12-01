pub mod buffer;
pub mod sampler;
pub mod texture;

use wgpu;

pub trait BindingLayout<T: Binding> {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry;

    fn create_binding(&self, device: &wgpu::Device) -> T;
}

pub trait Binding {
    fn get_resource(&self) -> wgpu::BindingResource;
}

