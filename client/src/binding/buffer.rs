use super::{Binding, BindingLayout};

struct BufferBindingLayout {
    binding: u32,
    visibility: wgpu::ShaderStage,
    binding_type: wgpu::BindingType,

    usage: wgpu::BufferUsage,
    buffer_size: wgpu::BufferAddress,
}

impl BindingLayout<BufferBinding> for BufferBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility,
            ty: self.binding_type.clone(),

            count: None,
        }
    }

    fn create_binding(&self, device: &wgpu::Device) -> BufferBinding {
        BufferBinding {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: self.buffer_size,
                usage: self.usage,
                mapped_at_creation: false,
            }),
        }
    }
}

struct BufferBinding {
    buffer: wgpu::Buffer,
}

impl Binding for BufferBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Buffer(self.buffer.slice(..))
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct UniformBindingLayout {
    buffer_binding_layout: BufferBindingLayout,
}

impl UniformBindingLayout {
    pub fn new<T>(binding: u32, visibility: wgpu::ShaderStage) -> UniformBindingLayout {
        UniformBindingLayout {
            buffer_binding_layout: BufferBindingLayout {
                binding,
                visibility,
                binding_type: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },

                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                buffer_size: std::mem::size_of::<T>() as wgpu::BufferAddress,
            },
        }
    }
}

impl BindingLayout<UniformBinding> for UniformBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        self.buffer_binding_layout.get_entry()
    }

    fn create_binding(&self, device: &wgpu::Device) -> UniformBinding {
        UniformBinding {
            buffer_binding: self.buffer_binding_layout.create_binding(device),
        }
    }
}

pub struct UniformBinding {
    buffer_binding: BufferBinding,
}

impl UniformBinding {
    pub fn update<T>(&self, data: &T, write_queue: &wgpu::Queue) {
        let raw_data: &[u8] = unsafe {
            std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of::<T>())
        };

        write_queue.write_buffer(&self.buffer_binding.buffer, 0, raw_data);
    }
}

impl Binding for UniformBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        self.buffer_binding.get_resource()
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct InstanceArrayBindingLayout {
    buffer_binding_layout: BufferBindingLayout,
}

impl InstanceArrayBindingLayout {
    pub fn new<T>(
        binding: u32,
        visibility: wgpu::ShaderStage,
        n_instances: u32,
    ) -> InstanceArrayBindingLayout {
        InstanceArrayBindingLayout {
            buffer_binding_layout: BufferBindingLayout {
                binding,
                visibility,
                binding_type: wgpu::BindingType::StorageBuffer {
                    dynamic: false,
                    min_binding_size: None,
                    readonly: false,
                },

                usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
                buffer_size: (std::mem::size_of::<T>() as u32 * n_instances) as wgpu::BufferAddress,
            },
        }
    }
}

impl BindingLayout<InstanceArrayBinding> for InstanceArrayBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        self.buffer_binding_layout.get_entry()
    }

    fn create_binding(&self, device: &wgpu::Device) -> InstanceArrayBinding {
        InstanceArrayBinding {
            buffer_binding: self.buffer_binding_layout.create_binding(device),
        }
    }
}

pub struct InstanceArrayBinding {
    buffer_binding: BufferBinding,
}

impl InstanceArrayBinding {
    pub fn update<T>(&self, data: &Vec<T>, write_queue: &wgpu::Queue) {
        let raw_data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                data.as_slice().as_ptr() as *const u8,
                std::mem::size_of::<T>(),
            )
        };

        write_queue.write_buffer(&self.buffer_binding.buffer, 0, raw_data);
    }
}

impl Binding for InstanceArrayBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        self.buffer_binding.get_resource()
    }
}
