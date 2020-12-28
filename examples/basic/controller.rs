use super::application::BasicApplication;
use brics::{
    application::ApplicationController, handle::camera::CameraHandle, pipeline::Geometry, script::*,
};
use cgmath::Vector3;

pub struct BasicController {
    scripts: Vec<Box<dyn Script<BasicApplication>>>,
}

impl ApplicationController<BasicApplication> for BasicController {
    fn new(app: &mut BasicApplication) -> Self {
        /*
        let geometry = self.get_cube_geometry();

        let cube = self.visual.create_shape_entity(&geometry);
        cube.get().translate(Vector3::new(0.0, 0.5, 0.0));
        cube.get().set_color(Vector3::new(0.2, 0.8, 0.2));
        cube.get().rescale(Vector3::new(0.5, 0.5, 0.5));
        self.add_controller(move |_| {
            cube.get().rotate(Vector3::new(0.2, 0.5, 0.9), 0.01);
        });

        let light = self.visual.get_light();
        let light_camera = self.visual.get_light_camera();
        let camera = self.visual.get_main_camera();
        self.add_controller(move |_| {
            light_camera
                .get()
                .look_at_dir(camera.get().get_center(), -light.get().get_direction());
        });

        let camera = self.visual.get_main_camera();
        let angle_multiplier = 0.004;
        let mut previous_mouse_input: Option<winit::dpi::PhysicalPosition<f64>> = None;
        self.add_controller(move |input: &InputState| match input.mouse.button {
            Some(_) => {
                if let Some(previous) = previous_mouse_input {
                    let delta_x = input.mouse.location.x - previous.x;
                    let delta_y = input.mouse.location.y - previous.y;

                    camera.get().rotate_around_center(
                        -angle_multiplier * delta_x as f32,
                        -angle_multiplier * delta_y as f32,
                    );
                }

                previous_mouse_input = Some(input.mouse.location);
            }
            None => previous_mouse_input = None,
        });
        */

        let geometry = app.get_cube_geometry();
        create_ground(app, &geometry);

        Self {
            scripts: vec![Box::new(get_main_camera_script(app))],
        }
    }

    fn step(&mut self, app: &mut BasicApplication) {
        for script in self.scripts.iter_mut() {
            script.update(app);
        }
    }
}

fn create_ground(app: &mut BasicApplication, geometry: &Geometry) {
    let ground = app.visual.create_shape_entity(&geometry);
    ground.rescale(Vector3::new(5.0, 0.02, 5.0));
    ground.translate(Vector3::new(0.0, -0.5, 0.0));
}

fn get_main_camera_script(
    app: &mut BasicApplication,
) -> ObjectController<CameraHandle, BasicApplication> {
    let angle_multiplier = 0.004;
    let mut previous_mouse_input: Option<winit::dpi::PhysicalPosition<f64>> = None;

    ObjectController::new(
        app.visual.get_main_camera(),
        move |cam, app: &mut BasicApplication| {
            let input = &app.input_state;
            match input.mouse.button {
                Some(_) => {
                    if let Some(previous) = previous_mouse_input {
                        let delta_x = input.mouse.location.x - previous.x;
                        let delta_y = input.mouse.location.y - previous.y;

                        cam.rotate_around_center(
                            -angle_multiplier * delta_x as f32,
                            -angle_multiplier * delta_y as f32,
                        );
                    }

                    previous_mouse_input = Some(input.mouse.location);
                }
                None => previous_mouse_input = None,
            };
        },
    )
}
