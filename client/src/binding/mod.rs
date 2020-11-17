mod uniform;
mod storage;

use wgpu;

pub trait BindingLayout<T: Binding> {
    fn get_layout_entry(&self) -> &wgpu::BindGroupLayoutEntry;

    fn create_binding(&self) -> T;
}

pub trait Binding {
    fn get_resource(&self) -> wgpu::BindingResource;
}

/*--------------------------------------------------------------------------------------------------*/

pub trait DynamicBuffer<T> {
    fn update(&mut self, data: &T, write_queue: &wgpu::Queue) {
        let raw_data =
            unsafe { std::slice::from_raw_parts(data as *const u8, std::mem::size_of::<T>()) };

        write_queue.write_buffer(self.get_buffer(), 0, raw_data)
    }

    fn get_buffer(&self) -> &wgpu::Buffer;
}

pub trait DynamicArrayBuffer<T> {
    fn update(&mut self, data: &Vec<T>, write_queue: &wgpu::Queue) {
        let raw_data = unsafe {
            std::slice::from_raw_parts(
                data.as_slice().as_ptr() as *const u8,
                std::mem::size_of::<T>(),
            )
        };

        write_queue.write_buffer(self.get_buffer(), 0, raw_data)
    }

    fn get_buffer(&self) -> &wgpu::Buffer;
}
