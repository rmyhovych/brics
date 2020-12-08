use crate::handle::object::{ObjectHandle, ObjectState};
use cgmath::{Matrix4, Vector3};
use std::{cell::RefCell, rc::Rc};

pub struct Object {
    state: ObjectState,

    handle: Rc<RefCell<ObjectHandle>>,
    instance: u32,
}

impl Object {
    pub fn new(handle: &Rc<RefCell<ObjectHandle>>, instance: u32) -> Self {
        let handle = Rc::clone(handle);
        let state = *(handle.borrow_mut().get_state(instance));

        Self {
            state,

            handle,
            instance,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.state.model = Matrix4::from_translation(Vector3 { x, y, z }) * self.state.model;

        self
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.state.color = Vector3 { x: r, y: g, z: b };

        self
    }

    pub fn rescale(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.state.model = self.state.model * Matrix4::from_nonuniform_scale(x, y, z);

        self
    }

    pub fn update_handle(&self) {
        self.handle
            .borrow_mut()
            .set_state(self.state, self.instance);
    }
}

pub struct Controller {
    object: Object,
    script: Box<dyn Fn(&mut Object)>,
}

impl Controller {
    pub fn new(
        handle: &Rc<RefCell<ObjectHandle>>,
        instance: u32,
        script: Box<dyn Fn(&mut Object)>,
    ) -> Self {
        Self {
            object: Object::new(handle, instance),
            script,
        }
    }

    pub fn update(&mut self) {
        self.script.as_ref()(&mut self.object);
        self.object.update_handle();
    }
}
