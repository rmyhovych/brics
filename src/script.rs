use super::application::Application;
use super::rcmut::RcMut;

use std::marker::PhantomData;

use std::borrow::BorrowMut;

pub trait Script {
    fn update(&mut self);
}

/*------------------------------------------------------------------------*/

pub struct ApplicationController<A: Application> {
    app: A,
    controller: Box<dyn FnMut(&mut A) + 'static>,
}

impl<A: Application> ApplicationController<A> {
    pub fn new(controller: impl FnMut(&mut A) + 'static) -> Self {
        Self {
            
            controller: Box::new(controller),
        }
    }
}

impl<A: Application> Script<A> for ApplicationController<A> {
    fn update(&mut self) {
        self.controller.as_mut()(app);
    }
}

/*------------------------------------------------------------------------*/

pub trait Object {}

pub struct ObjectController<O: Object> {
    object: RcMut<O>,
    controller: Box<dyn FnMut(&mut O) + 'static>,
}

impl<O: Object> ObjectController<O> {
    pub fn new(object: RcMut<O>, controller: impl FnMut(&mut O) + 'static) -> Self {
        Self {
            object,
            controller: Box::new(controller),
        }
    }
}

impl<O: Object> Script for ObjectController<O> {
    fn update(&mut self) {
        self.controller.as_mut()(&mut self.object);
    }
}
