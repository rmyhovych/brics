pub mod buffer;
pub mod sampler;
pub mod texture;

pub mod types;

use wgpu;

pub trait BindingLayout<T: Binding> {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry;

    fn create_binding(&self, device: &wgpu::Device) -> T;
}

pub trait Binding {
    fn get_resource(&self) -> wgpu::BindingResource;
}

/*----------------------------------------------------------------------------------*/

pub trait BindingLayoutHandle<B: Binding, L: BindingLayout<B>> {
    fn get_binding_layout(&self) -> &L;
}

pub trait BindingHandle<B: Binding> {
    fn get_binding(&self) -> &B;

    fn update(&self, write_queue: &wgpu::Queue);
}
