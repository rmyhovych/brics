use super::{Binding, BindingLayout, DynamicBuffer};

pub struct UniformBindingLayout<'a, T> {
    device: &'a wgpu::Device,
    layout_entry: wgpu::BindGroupLayoutEntry,

    buffer_usage: wgpu::BufferUsage,
}

impl<'a, T> BindingLayout<UniformBinding> for UniformBindingLayout<'a, T> {
    fn get_layout_entry(&self) -> &wgpu::BindGroupLayoutEntry {
        &self.layout_entry
    }

    fn create_binding(&self) -> UniformBinding {
        UniformBinding {
            uniform_buffer: self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: std::mem::size_of::<T>(),
                usage: self.buffer_usage,
                mapped_at_creation: true,
            }),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct UniformBinding<T> {
    uniform_buffer: wgpu::Buffer,
}

impl Binding for UniformBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer(self.uniform_buffer.slice(..))
    }
}

impl<T> DynamicBuffer<T> for UniformBinding<T> {
    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.uniform_buffer
    }
}
