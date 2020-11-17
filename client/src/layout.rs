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

pub trait LayoutHandler {
    fn get_bind_group_layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry>;

    fn get_bind_group_entries(&self) -> Vec<wgpu::BindGroupEntry>;

    fn write_buffer<'a>(&'a self, write_queue: &wgpu::Queue, buffer: &wgpu::Buffer, data: &T) {
        write_queue.write_buffer(buffer, 0, bytemuck::cast_slice(data.as_ref()));
    }

    fn apply_on_renderpass<'a>(
        &'a self,
        renderpass: &mut wgpu::RenderPass<'a>,
        write_queue: &wgpu::Queue,
    );
}
