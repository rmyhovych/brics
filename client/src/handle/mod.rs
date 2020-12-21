pub mod camera;
pub mod light;
pub mod sampler;
pub mod shape;
pub mod texture;

use crate::{
    binding::{Binding, BindingLayout},
    renderer,
};
use renderer::Renderer;

/*----------------------------------------------------------------------------------*/

pub trait BindingHandleLayout<B: Binding, L: BindingLayout<B>, H: BindingHandle> {
    fn get_binding_layout(&self) -> &L;

    fn create_handle(&self, renderer: &Renderer) -> H;
}

pub trait BindingHandle {
    fn get_binding(&self) -> &dyn Binding;

    fn update(&self, write_queue: &wgpu::Queue);
}
