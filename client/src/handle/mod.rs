pub mod camera;
pub mod light;
pub mod sampler;
pub mod shape;
pub mod texture;

use crate::{
    binding::{Binding, BindingLayout},
    graphics,
};
use graphics::GraphicsManager;

/*----------------------------------------------------------------------------------*/

pub trait BindingHandleLayout<B: Binding, L: BindingLayout<B>, H: BindingHandle> {
    fn get_binding_layout(&self) -> &L;

    fn create_handle(&self, graphics: &GraphicsManager) -> H;
}

pub trait BindingHandle {
    fn get_binding(&self) -> &dyn Binding;

    fn update(&self, write_queue: &wgpu::Queue);
}

/*----------------------------------------------------------------------------------*/
pub struct BindingProxy<H: BindingHandle> {
    mem: *mut H,
}

impl<H: BindingHandle> BindingProxy<H> {
    pub fn new(binding: &mut H) -> Self {
        Self { mem: binding }
    }

    pub fn get(&self) -> &mut H {
        unsafe { self.mem.as_mut().unwrap() }
    }
}
