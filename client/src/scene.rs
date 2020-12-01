use crate::input::InputState;
use crate::renderer::Renderer;

pub trait Scene {
    fn game_loop(&mut self, input_state: &InputState, renderer: &mut Renderer);
}
