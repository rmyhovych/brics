use super::{Binding, BindingLayout};

struct BufferBindingLayout {
    visibility: wgpu::ShaderStage,

    usage: wgpu::BufferUsage,
    buffer_size: wgpu::BufferAddress,
}

impl BufferBindingLayout {
    fn create_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: self.buffer_size,
            usage: self.usage,
            mapped_at_creation: false,
        })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct UniformBindingLayout {
    buffer_binding_layout: BufferBindingLayout,
}

impl UniformBindingLayout {
    pub fn new<T>(visibility: wgpu::ShaderStage) -> UniformBindingLayout {
        UniformBindingLayout {
            buffer_binding_layout: BufferBindingLayout {
                visibility,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                buffer_size: std::mem::size_of::<T>() as wgpu::BufferAddress,
            },
        }
    }
}

impl BindingLayout<UniformBinding> for UniformBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: self.buffer_binding_layout.visibility,
            ty: wgpu::BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: None,
            },

            count: None,
        }
    }

    fn create_binding(&self, device: &wgpu::Device) -> UniformBinding {
        UniformBinding {
            buffer: self.buffer_binding_layout.create_buffer(device),
        }
    }
}

pub struct UniformBinding {
    buffer: wgpu::Buffer,
}

impl UniformBinding {
    pub fn update<T>(&self, data: &T, write_queue: &wgpu::Queue) {
        let raw_data: &[u8] = unsafe {
            std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of::<T>())
        };

        write_queue.write_buffer(&self.buffer, 0, raw_data);
    }
}

impl Binding for UniformBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer(self.buffer.slice(..))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct InstanceArrayBindingLayout {
    buffer_binding_layout: BufferBindingLayout,
}

impl InstanceArrayBindingLayout {
    pub fn new<T>(visibility: wgpu::ShaderStage, n_instances: u32) -> InstanceArrayBindingLayout {
        InstanceArrayBindingLayout {
            buffer_binding_layout: BufferBindingLayout {
                visibility,

                usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
                buffer_size: (std::mem::size_of::<T>() as u32 * n_instances) as wgpu::BufferAddress,
            },
        }
    }
}

impl BindingLayout<InstanceArrayBinding> for InstanceArrayBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: self.buffer_binding_layout.visibility,
            ty: wgpu::BindingType::StorageBuffer {
                dynamic: false,
                min_binding_size: None,
                readonly: false,
            },

            count: None,
        }
    }

    fn create_binding(&self, device: &wgpu::Device) -> InstanceArrayBinding {
        InstanceArrayBinding {
            buffer: self.buffer_binding_layout.create_buffer(device),
        }
    }
}

pub struct InstanceArrayBinding {
    buffer: wgpu::Buffer,
}

impl InstanceArrayBinding {
    pub fn update<T>(&self, data: &Vec<T>, write_queue: &wgpu::Queue) {
        let raw_data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                data.as_slice().as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
        };

        write_queue.write_buffer(&self.buffer, 0, raw_data);
    }
}

impl Binding for InstanceArrayBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer(self.buffer.slice(..))
    }
}
