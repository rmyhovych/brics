use crate::renderer::Renderer;

pub trait GameLogic<R: Renderer> {
    fn new(renderer: &mut R) -> Self;

    fn step(&mut self);
}
