use crate::input::InputState;

pub trait Renderer {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self
    where
        Self: Sized;

    fn render(&mut self);

    fn request_redraw(&self);
}
