use crate::input::InputState;
use crate::renderer::Renderer;

pub trait Scene {
    fn new(renderer: &mut Renderer) -> Self;

    fn game_loop(&mut self, input_state: &InputState, renderer: &mut Renderer);
}
