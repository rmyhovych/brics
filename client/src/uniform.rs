use std::mem::transmute;

pub trait Uniform: Sized {
    fn as_ref(&self) -> &[u8] {
        unsafe { return transmute(*self) }
    }

    fn size() -> u64 {
        std::mem::size_of::<Self>() as u64
    }
}

pub trait UniformDescriptor<T: Uniform> {
    fn get_bind_group_layout_entry() -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 1,
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
            binding: 1,
            resource: wgpu::BindingResource::Buffer(self.get_uniform_buffer().slice(..)),
        }
    }

    fn update_uniform(&mut self, write_queue: &wgpu::Queue, data: &T) {
        write_queue.write_buffer(
            self.get_uniform_buffer(),
            0,
            bytemuck::cast_slice(data.as_ref()),
        );
    }

    fn get_uniform_buffer(&self) -> &wgpu::Buffer;

    fn apply_on_renderpass<'a>(
        &'a self,
        renderpass: &wgpu::RenderPass<'a>,
        write_queue: &wgpu::Queue,
    );
}
