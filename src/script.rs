use super::application::Application;
use super::handle::{BindingHandle, BindingProxy};

pub trait Script<A: Application> {
    fn update(&mut self, app: &mut A);
}

/*------------------------------------------------------------------------*/

pub struct LogicScript<A: Application> {
    controller: Box<dyn FnMut(&mut A) + 'static>,
}

impl<A: Application> LogicScript<A> {
    pub fn new(controller: impl FnMut(&mut A) + 'static) -> Self {
        Self {
            controller: Box::new(controller),
        }
    }
}

impl<A: Application> Script<A> for LogicScript<A> {
    fn update(&mut self, app: &mut A) {
        self.controller.as_mut()(app);
    }
}

/*------------------------------------------------------------------------*/

pub struct ObjectController<B: BindingHandle, A: Application> {
    object: BindingProxy<B>,
    controller: Box<dyn FnMut(&mut B, &mut A) + 'static>,
}

impl<B: BindingHandle, A: Application> ObjectController<B, A> {
    pub fn new(object: &mut B, controller: impl FnMut(&mut B, &mut A) + 'static) -> Self {
        Self {
            object: BindingProxy::new(object),
            controller: Box::new(controller),
        }
    }
}

impl<B: BindingHandle, A: Application> Script<A> for ObjectController<B, A> {
    fn update(&mut self, app: &mut A) {
        self.controller.as_mut()(self.object.get(), app);
    }
}
