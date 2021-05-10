use super::application::BasicApplication;
use brics::{
    application::ApplicationController,
    handle::{camera::CameraHandle, shape::ShapeHandle},
    pipeline::Geometry,
    script::*,
};
use cgmath::Vector3;

pub struct BasicController {
    scripts: Vec<Box<dyn Script<BasicApplication>>>,
}

impl ApplicationController<BasicApplication> for BasicController {
    fn new(app: &mut BasicApplication) -> Self {
        let geometry = app.get_cube_geometry();
        create_ground(app, &geometry);

        Self {
            scripts: vec![
                Box::new(get_main_camera_script(app)),
                Box::new(get_cube_script(app, &geometry)),
                Box::new(get_light_camera_script()),
            ],
        }
    }

    fn step(&mut self, app: &mut BasicApplication, _: f32) {      
        for script in self.scripts.iter_mut() {
            script.update(app);
        }
    }
}

fn create_ground(app: &mut BasicApplication, geometry: &Geometry) {
    let ground = app.visual.create_shape_entity(&geometry);
    ground.borrow_mut().rescale(Vector3::new(5.0, 0.02, 5.0));
    ground.borrow_mut().translate(Vector3::new(0.0, -0.5, 0.0));
}

fn get_main_camera_script(
    app: &mut BasicApplication,
) -> ObjectController<CameraHandle, BasicApplication> {
    let angle_multiplier = 0.004;
    let mut previous_mouse_input: Option<winit::dpi::PhysicalPosition<f64>> = None;

    ObjectController::new(
        app.visual.get_main_camera(),
        move |mut cam, app: &mut BasicApplication| {
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

fn get_light_camera_script() -> LogicScript<BasicApplication> {
    LogicScript::new(|app: &mut BasicApplication| {
        app.visual.get_light_camera().borrow_mut().look_at_dir(
            app.visual.get_main_camera().borrow_mut().get_center(),
            -app.visual.get_light().borrow_mut().get_direction(),
        );
    })
}

fn get_cube_script(
    app: &mut BasicApplication,
    geometry: &Geometry,
) -> ObjectController<ShapeHandle, BasicApplication> {
    let cube = app.visual.create_shape_entity(geometry);
    {
        let mut cube_ref = cube.borrow_mut();
        cube_ref.translate(Vector3::new(0.0, 0.5, 0.0));
        cube_ref.set_color(Vector3::new(0.2, 0.8, 0.2));
        cube_ref.rescale(Vector3::new(0.5, 0.5, 0.5));
    }

    ObjectController::new(cube, move |mut cube, app: &mut BasicApplication| {
        cube.rotate(Vector3::new(0.2, 0.5, 0.9), 0.01);
    })
}
