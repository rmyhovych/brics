use crate::renderer::Renderer;

use winit;

pub trait GameLogic<R: Renderer> {
    fn new() -> Self;

    fn setup(&mut self, renderer: &mut R);

    fn step(&mut self);

    fn handle_input(&mut self, event: &winit::event::WindowEvent);
}
