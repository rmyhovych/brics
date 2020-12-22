use cgmath::Vector3;

use crate::handle::{camera::CameraHandle, light::LightHandle, shape::ShapeHandle};

/*--------------------------------------------------------------------------------------------------*/

struct BindingProxy<H: DynamicBinding> {
    mem: *mut H,
}

impl<H: DynamicBinding> BindingProxy<H> {
    pub fn new(binding: &mut H) -> Self {
        Self { mem: binding }
    }

    pub fn get(&self) -> &mut H {
        unsafe { self.mem.as_mut().unwrap() }
    }
}

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

pub trait DynamicBinding {
    fn apply_changes(&mut self, transform: &Transform);
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Object {
    pub transform: Transform,

    shape: Option<BindingProxy<ShapeHandle>>,
    light: Option<BindingProxy<LightHandle>>,
    camera: Option<BindingProxy<CameraHandle>>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),

            shape: None,
            light: None,
            camera: None,
        }
    }

    pub fn set_shape(&mut self, shape: &mut ShapeHandle) {
        self.shape = Self::wrap(shape);
    }

    pub fn set_light(&mut self, light: &mut LightHandle) {
        self.light = Self::wrap(light);
    }

    pub fn set_camera(&mut self, camera: &mut CameraHandle) {
        self.camera = Self::wrap(camera);
    }

    pub fn apply_changes(&mut self) {
        self.apply(&self.shape);
        self.apply(&self.light);
        self.apply(&self.camera);
    }

    fn wrap<H: DynamicBinding>(handle: &mut H) -> Option<BindingProxy<H>> {
        Some(BindingProxy::new(handle))
    }

    fn apply<H: DynamicBinding>(&self, handle: &Option<BindingProxy<H>>) {
        if let Some(proxy) = handle {
            proxy.get().apply_changes(&self.transform);
        }
    }
}
