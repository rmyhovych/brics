extern crate rustgame;
use rustgame::*;

use cgmath::{self, InnerSpace};

use binding::{Binding, BindingLayout};
use resource::DynamicResource;

use wgpu;
use winit::{self, event::VirtualKeyCode};

struct VertexBasic {
    position: cgmath::Point3<f32>,
    normal: cgmath::Vector3<f32>,
}

impl pipeline::Vertex for VertexBasic {
    fn get_attribute_formats() -> Vec<wgpu::VertexFormat> {
        vec![wgpu::VertexFormat::Float3, wgpu::VertexFormat::Float3]
    }
}

struct Model {
    model: cgmath::Matrix4<f32>,
    color: cgmath::Vector3<f32>,
}

pub struct RedrawHandler {
    #[cfg(not(target_arch = "wasm32"))]
    previous: std::time::Instant,
}

impl RedrawHandler {
    pub fn new() -> Self {
        Self {
            previous: std::time::Instant::now(),
        }
    }

    pub fn request(&mut self, app: &application::Application) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.previous.elapsed() > std::time::Duration::from_millis(16) {
                app.request_redraw();
                self.previous = std::time::Instant::now();
            }
        }

        #[cfg(target_arch = "wasm32")]
        app.request_redraw();
    }
}

fn setup(app: &mut application::Application) -> impl FnMut(&wgpu::Queue, &input::InputState) {
    let window_size = app.get_window_size();
    let mut camera = camera::Camera::look_at(
        &app.device,
        &cgmath::Point3 {
            x: 5.0,
            y: 5.0,
            z: -5.5,
        },
        &cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        window_size.width as f32 / window_size.height as f32,
    );

    let model = Model {
        model: cgmath::Matrix4::from_scale(1.0),
        color: cgmath::Vector3 {
            x: 0.5,
            y: 0.2,
            z: 0.5,
        },
    };
    let object_binding_layout =
        binding::buffer::UniformBindingLayout::new::<Model>(1, wgpu::ShaderStage::VERTEX);
    let object_binding = object_binding_layout.create_binding(&app.device);

    // ADD ENTRIES
    let mut binding_entries = pipeline::BindingEntries::new();
    binding_entries
        .add(camera.get_binding_layout())
        .add(&object_binding_layout);

    let (vertices, indices) = create_vertices();
    app.create_pipeline::<VertexBasic>(
        "examples/shaders/shader.vert",
        "examples/shaders/shader.frag",
        binding_entries,
        &vec![pipeline::EntityDescriptor {
            vertices,
            indices,
            bindings: vec![camera.get_binding(), &object_binding],
            n_instances: 1,
        }],
    );

    let mut previous_mouse_input: Option<winit::dpi::PhysicalPosition<f64>> = None;
    let angle_multiplier = 0.004;
    let movement_speed = 0.1;

    /*-------------------------- GAME LOOP --------------------------*/

    move |queue: &wgpu::Queue, input_state: &input::InputState| {
        match input_state.mouse.button {
            Some(_) => {
                if let Some(previous) = previous_mouse_input {
                    let delta_x = input_state.mouse.location.x - previous.x;
                    let delta_y = input_state.mouse.location.y - previous.y;

                    camera.rotate_direction(
                        -angle_multiplier * delta_x as f32,
                        angle_multiplier * delta_y as f32,
                    );
                }

                previous_mouse_input = Some(input_state.mouse.location);
            }
            None => previous_mouse_input = None,
        }

        {
            let keyboard_pressed = &input_state.keyboard.pressed;

            let direction = camera.get_direction().normalize();
            let right = direction.cross(cgmath::Vector3::unit_y()).normalize();

            let mut movement = cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            if keyboard_pressed.contains(&VirtualKeyCode::W) {
                movement += direction;
            }
            if keyboard_pressed.contains(&VirtualKeyCode::S) {
                movement -= direction;
            }
            if keyboard_pressed.contains(&VirtualKeyCode::D) {
                movement += right;
            }
            if keyboard_pressed.contains(&VirtualKeyCode::A) {
                movement -= right;
            }

            if movement.magnitude2() > 0.0 {
                movement = movement.normalize_to(movement_speed);
            }

            camera.translate(movement.x, movement.y, movement.z);
        }

        camera.update(queue);
        object_binding.update(&model, queue);
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut app: application::Application =
        futures::executor::block_on(application::Application::new(&event_loop));

    let mut game_loop = setup(&mut app);

    let mut swap_chain = app.create_swap_chain();

    let mut input_state = input::InputState::new();

    let mut redraw_handler = RedrawHandler::new();
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            redraw_handler.request(&app);
        }
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => {
                input_state.handle(&event);
            }
        },
        winit::event::Event::Suspended | winit::event::Event::Resumed => {
            println!("EVENT [{:?}]", event);
        }
        winit::event::Event::RedrawRequested(_) => {
            let frame = match swap_chain.get_current_frame() {
                Ok(frame) => frame,
                Err(_) => {
                    swap_chain = app.create_swap_chain();
                    swap_chain
                        .get_current_frame()
                        .expect("Failed to acquire next swap chain texture!")
                }
            };

            app.step(&mut game_loop, &input_state);
            app.render(&frame);
        }
        _ => {}
    });
}

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
    .map(|raw_vertex| VertexBasic {
        position: cgmath::Point3 {
            x: raw_vertex[0],
            y: raw_vertex[1],
            z: raw_vertex[2],
        },
        normal: cgmath::Vector3 {
            x: raw_vertex[3],
            y: raw_vertex[4],
            z: raw_vertex[5],
        },
    })
    .collect::<Vec<VertexBasic>>();
    let index_data = [
        0, 1, 3, 3, 1, 2, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ]
    .to_vec();

    return (vertex_data, index_data);
}
