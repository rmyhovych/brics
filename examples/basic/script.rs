use brics::application::Application;
use brics::handle::BindingHandle;

pub trait Script<A: Application> {
    fn update(&mut self, app: &mut A);
}

/*------------------------------------------------------------------------*/

pub struct LogicScript<A: Application> {
    controller: Box<dyn Fn(&mut A) + 'static>,
}

impl<A: Application> LogicScript<A> {
    pub fn new(controller: impl Fn(&mut A) + 'static) -> Self {
        Self {
            controller: Box::new(controller),
        }
    }
}

impl<A: Application> Script<A> for LogicScript<A> {
    fn update(&mut self, app: &mut A) {
        self.controller.as_ref()(app);
    }
}

/*------------------------------------------------------------------------*/

pub struct ObjectController<'a, B: BindingHandle, A: Application> {
    object: &'a mut B,
    controller: Box<dyn Fn(&mut B, &mut A) + 'static>,
}

impl<'a, B: BindingHandle, A: Application> ObjectController<'a, B, A> {
    pub fn new(object: &'a mut B, controller: impl Fn(&mut B, &mut A) + 'static) -> Self {
        Self {
            object,
            controller: Box::new(controller),
        }
    }
}

impl<'a, B: BindingHandle, A: Application> Script<A> for ObjectController<'a, B, A> {
    fn update(&mut self, app: &mut A) {
        self.controller.as_ref()(&mut self.object, app);
    }
}
