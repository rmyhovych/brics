use cgmath::Vector3;

/*--------------------------------------------------------------------------------------------------*/

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn translate(&mut self, delta: Vector3<f32>) -> &mut Self {
        self.position += delta;

        self
    }
}

pub trait Transformable {
    fn accept_transform(&mut self, transform: &Transform);
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Object {
    transform: Transform,

    object_id: Option<u32>,
    light_id: Option<u32>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            object_id: None,
            light_id: None,
        }
    }

    pub fn set_object(&mut self, id: u32) {
        self.object_id = Some(id);
    }

    pub fn set_light(&mut self, id: u32) {
        self.light_id = Some(id);
    }

    pub fn apply(&self, transformable: &mut dyn Transformable) {
        transformable.accept_transform(&self.transform);
    }
}

pub struct Controller {
    pub object: Object,
    update_action: Box<dyn Fn(&mut Object)>,
}

impl Controller {
    pub fn new<F>(object: Object, action: impl Fn(&mut Object) + 'static) -> Self {
        let update_action =  Box::new(action);
        Controller {
            object,
            update_action,
        }
    }

    pub fn update(&mut self) {
        (self.update_action)(&mut self.object);
    }
}
