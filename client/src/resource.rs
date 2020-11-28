use wgpu;

pub trait DynamicResource {
    fn update(&self, write_queue: &wgpu::Queue);
}
