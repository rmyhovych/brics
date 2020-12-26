use super::{vertex::VertexBasic, visual::MainVisual};

use rustgame::{
    application::{GameLogic, Visual},
    input::InputState,
    pipeline::Geometry,
};

use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

use winit::event::{VirtualKeyCode, WindowEvent};

/*--------------------------------------------------------------------------------------------------*/

fn create_vertices() -> (Vec<VertexBasic>, Vec<u16>) {
    let vertex_data = [
        // Back face
        [-1.0, -1.0, 1.0, 0.0, 0.0, 1.0],
        [1.0, -1.0, 1.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 0.0, 0.0, 1.0],
        [-1.0, 1.0, 1.0, 0.0, 0.0, 1.0],
        // Front face
        [-1.0, -1.0, -1.0, 0.0, 0.0, -1.0],
        [-1.0, 1.0, -1.0, 0.0, 0.0, -1.0],
        [1.0, 1.0, -1.0, 0.0, 0.0, -1.0],
        [1.0, -1.0, -1.0, 0.0, 0.0, -1.0],
        // Bottom face
        [-1.0, 1.0, -1.0, 0.0, 1.0, 0.0],
        [-1.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        [1.0, 1.0, -1.0, 0.0, 1.0, 0.0],
        // Top face
        [-1.0, -1.0, -1.0, 0.0, -1.0, 0.0],
        [1.0, -1.0, -1.0, 0.0, -1.0, 0.0],
        [1.0, -1.0, 1.0, 0.0, -1.0, 0.0],
        [-1.0, -1.0, 1.0, 0.0, -1.0, 0.0],
        // Right face
        [1.0, -1.0, -1.0, 1.0, 0.0, 0.0],
        [1.0, 1.0, -1.0, 1.0, 0.0, 0.0],
        [1.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        [1.0, -1.0, 1.0, 1.0, 0.0, 0.0],
        // Left face
        [-1.0, -1.0, -1.0, -1.0, 0.0, 0.0],
        [-1.0, -1.0, 1.0, -1.0, 0.0, 0.0],
        [-1.0, 1.0, 1.0, -1.0, 0.0, 0.0],
        [-1.0, 1.0, -1.0, -1.0, 0.0, 0.0],
    ]
    .iter()
    .map(|raw_vertex| {
        VertexBasic::new(
            cgmath::Point3 {
                x: raw_vertex[0],
                y: raw_vertex[1],
                z: raw_vertex[2],
            },
            cgmath::Vector3 {
                x: raw_vertex[3],
                y: raw_vertex[4],
                z: raw_vertex[5],
            },
        )
    })
    .collect::<Vec<VertexBasic>>();
    let index_data = [
        0, 1, 3, 3, 1, 2, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ]
    .to_vec();

    return (vertex_data, index_data);
}

/*--------------------------------------------------------------------------------------------------*/

pub struct MainLogic {
    input_state: InputState,

    controllers: Vec<Box<dyn FnMut(&InputState)>>,
}

impl GameLogic<MainVisual> for MainLogic {
    fn new() -> Self {
        Self {
            input_state: InputState::new(),

            controllers: Vec::new(),
        }
    }

    fn setup(&mut self, visual: &mut MainVisual) {
        let geometry = self.get_cube_geometry(visual);

        let ground = visual.create_shape_entity(&geometry);
        ground.get().rescale(Vector3::new(5.0, 0.02, 5.0));
        ground.get().translate(Vector3::new(0.0, -0.5, 0.0));

        let cube = visual.create_shape_entity(&geometry);
        cube.get().translate(Vector3::new(0.0, 0.5, 0.0));
        cube.get().set_color(Vector3::new(0.2, 0.8, 0.2));
        cube.get().rescale(Vector3::new(0.5, 0.5, 0.5));
        self.add_controller(move |_| {
            cube.get().rotate(Vector3::new(0.2, 0.5, 0.9), 0.01);
        });

        let light = visual.get_light();
        let light_camera = visual.get_light_camera();
        let camera = visual.get_main_camera();
        self.add_controller(move |_| {
            light_camera
                .get()
                .look_at_dir(camera.get().get_center(), -light.get().get_direction());
        });

        let camera = visual.get_main_camera();
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
    }

    fn step(&mut self) {
        for controller in &mut self.controllers {
            controller(&self.input_state);
        }
    }

    fn handle_input(&mut self, event: &WindowEvent) {
        self.input_state.handle(event);
    }
}

impl MainLogic {
    fn get_cube_geometry(&self, visual: &mut MainVisual) -> Geometry {
        let (vertices, indices) = create_vertices();
        visual.create_geometry(vertices, indices)
    }

    fn add_controller<C: FnMut(&InputState) + 'static>(&mut self, ctrl: C) {
        self.controllers.push(Box::new(ctrl));
    }
}
