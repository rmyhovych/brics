use crate::input::InputState;
use crate::renderer::Renderer;

pub trait Scene {
    fn new(renderer: &mut Renderer) -> Self;

    fn setup_logic(&mut self, renderer: &mut Renderer);

    fn step(&mut self, input_state: &InputState, renderer: &mut Renderer);
}
