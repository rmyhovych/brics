pub trait Uniform: Sized {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn size() -> u64 {
        std::mem::size_of::<Self>() as u64
    }
}

pub trait UniformDescriptor<T: Uniform> {
    fn get_bind_group_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.get_binding(),
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: wgpu::BufferSize::new(T::size()),
            },
            count: None,
        }
    }

    fn get_bind_group_entry(&self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: self.get_binding(),
            resource: wgpu::BindingResource::Buffer(self.get_uniform_buffer().slice(..)),
        }
    }

    fn write_uniform<'a>(&'a self, write_queue: &wgpu::Queue, data: &T) {
        write_queue.write_buffer(
            self.get_uniform_buffer(),
            0,
            bytemuck::cast_slice(data.as_ref()),
        );
    }

    fn get_binding(&self) -> u32;

    fn get_uniform_buffer(&self) -> &wgpu::Buffer;

    fn apply_on_renderpass<'a>(
        &'a self,
        renderpass: &mut wgpu::RenderPass<'a>,
        write_queue: &wgpu::Queue,
    );
}
