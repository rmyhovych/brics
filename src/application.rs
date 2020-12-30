use winit;


pub trait Visual {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self
    where
        Self: Sized;

    fn render(&mut self);

    fn request_redraw(&self);
}

pub trait Application {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self
    where
        Self: Sized;

    fn handle_input(&mut self, event: &winit::event::WindowEvent);

    fn request_redraw(&self);

    fn step(&mut self);
}

pub trait ApplicationController<A: Application> {
    fn new(app: &mut A) -> Self;

    fn step(&mut self, app: &mut A, time_step: f32);
}
