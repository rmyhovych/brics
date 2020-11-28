extern crate rustgame;
use rustgame::*;

use binding::{Binding, BindingLayout};
use resource::DynamicResource;

use wgpu;
use winit;

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

fn setup(app: &mut application::Application) -> impl FnMut(&wgpu::Queue) {
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

    /*-------------------------- GAME LOOP --------------------------*/

    move |queue: &wgpu::Queue| {
        camera.rotate_around_center(0.01, 0.0);
        camera.update(queue);

        object_binding.update(&model, queue);
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let mut pool = futures::executor::LocalPool::new();

    let event_loop = winit::event_loop::EventLoop::new();
    let mut app: application::Application =
        futures::executor::block_on(application::Application::new(&event_loop));

    let mut game_loop = setup(&mut app);

    let mut swap_chain = app.create_swap_chain();

    #[cfg(not(target_arch = "wasm32"))]
    let mut last_update_inst = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if last_update_inst.elapsed() > std::time::Duration::from_millis(16) {
                    app.request_redraw();
                    last_update_inst = std::time::Instant::now();
                }

                pool.run_until_stalled();
            }

            #[cfg(target_arch = "wasm32")]
            app.request_redraw();
        }
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            winit::event::WindowEvent::Touch(touch) => match touch.phase {
                winit::event::TouchPhase::Started => {}
                winit::event::TouchPhase::Moved => {}
                _ => (),
            },
            _ => {}
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

            app.step(&mut game_loop);
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
