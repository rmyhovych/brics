use super::{Binding, BindingLayout, DynamicArrayBuffer};

pub struct StorageBindingLayout<'a, T> {
    device: &'a wgpu::Device,
    layout_entry: wgpu::BindGroupLayoutEntry,

    buffer_usage: wgpu::BufferUsage,
}

impl<'a, T> BindingLayout<StorageBinding> for StorageBindingLayout<'a, T> {
    fn get_layout_entry(&self) -> &wgpu::BindGroupLayoutEntry {
        &self.layout_entry
    }

    fn create_binding(&self) -> StorageBinding {
        StorageBinding {
            storage_buffer: self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: std::mem::size_of::<T>(),
                usage: self.buffer_usage,
                mapped_at_creation: true,
            }),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct StorageBinding<T> {
    storage_buffer: wgpu::Buffer,
}

impl Binding for StorageBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer(self.storage_buffer.slice(..))
    }
}

impl<T> DynamicArrayBuffer<T> for StorageBinding<T> {
    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.storage_buffer
    }
}
