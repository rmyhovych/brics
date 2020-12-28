use super::application::Application;
use super::handle::BindingHandle;
use std::{cell::RefCell, rc::Rc};

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
    object: Rc<RefCell<B>>,
    controller: Box<dyn FnMut(std::cell::RefMut<B>, &mut A) + 'static>,
}

impl<B: BindingHandle, A: Application> ObjectController<B, A> {
    pub fn new(
        object: Rc<RefCell<B>>,
        controller: impl FnMut(std::cell::RefMut<B>, &mut A) + 'static,
    ) -> Self {
        Self {
            object,
            controller: Box::new(controller),
        }
    }
}

impl<B: BindingHandle, A: Application> Script<A> for ObjectController<B, A> {
    fn update(&mut self, app: &mut A) {
        self.controller.as_mut()(self.object.borrow_mut(), app);
    }
}
