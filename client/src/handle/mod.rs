pub mod camera;
pub mod object;

use crate::binding::{Binding, BindingLayout};

/*----------------------------------------------------------------------------------*/

pub trait BindingLayoutHandle<B: Binding, L: BindingLayout<B>> {
    fn get_binding_layout(&self) -> &L;
}

pub trait BindingHandle<B: Binding> {
    fn get_binding(&self) -> &B;

    fn update(&self, write_queue: &wgpu::Queue);
}
