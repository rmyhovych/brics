use crate::visual;

use super::{vertex::VertexBasic, visual::BasicVisual};

use brics::{
    application::{Application, Visual},
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

pub struct BasicApplication {
    pub visual: BasicVisual,
    pub input_state: InputState,
}

impl Application for BasicApplication {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        Self {
            visual: BasicVisual::new(event_loop),
            input_state: InputState::new(),
        }
    }

    fn step(&mut self) {
        self.visual.render();
    }

    fn handle_input(&mut self, event: &WindowEvent) {
        self.input_state.handle(event);
    }

    fn request_redraw(&self) {
        self.visual.request_redraw();
    }
}

impl BasicApplication {
    pub fn get_cube_geometry(&self) -> Geometry {
        let (vertices, indices) = create_vertices();
        self.visual.create_geometry(vertices, indices)
    }
}
