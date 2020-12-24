use cgmath::Vector3;

use crate::handle::{camera::CameraHandle, light::LightHandle, shape::ShapeHandle, BindingHandle};

/*--------------------------------------------------------------------------------------------------*/

/*--------------------------------------------------------------------------------------------------*/

/*
pub struct Object {
    shape: Option<BindingProxy<ShapeHandle>>,
    light: Option<BindingProxy<LightHandle>>,
    camera: Option<BindingProxy<CameraHandle>>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            shape: None,
            light: None,
            camera: None,
        }
    }

    pub fn set_shape(&mut self, shape: &mut ShapeHandle) {
        self.shape = Self::wrap(shape);
    }

    pub fn get_shape(&self) -> Option<&mut ShapeHandle> {
        self.extract(self.shape)
    }

    pub fn set_light(&mut self, light: &mut LightHandle) {
        self.light = Self::wrap(light);
    }

    pub fn set_camera(&mut self, camera: &mut CameraHandle) {
        self.camera = Self::wrap(camera);
    }

    pub fn apply_changes(&mut self) {
    }

    fn wrap<H: BindingHandle>(handle: &mut H) -> Option<BindingProxy<H>> {
        Some(BindingProxy::new(handle))
    }

    fn extract<H: BindingHandle>(&self, proxy: &Option<BindingProxy<H>>) -> Option<&mut H> {
        match proxy {
            Some(h) => Some(h.get()),
            None => None,
        }
    }
}
*/
