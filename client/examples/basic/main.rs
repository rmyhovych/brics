extern crate rustgame;

use rustgame::{
    application::Application,
    handle::{
        camera::CameraHandle,
        light::LightHandle,
        object::{InstancedObjectHandle, Object, ObjectHandle},
        BindingHandle, BindingLayoutHandle,
    },
    render_pass::{RenderPass, AttachmentView},
    input::InputState,
    pipeline::{BindingEntries, EntityDescriptor, Vertex},
    renderer::Renderer,
    scene::Scene,
};

use cgmath::{self, InnerSpace};

use wgpu;
use winit::{self, event::VirtualKeyCode};

struct VertexBasic {
    position: cgmath::Point3<f32>,
    normal: cgmath::Vector3<f32>,
}

impl Vertex for VertexBasic {
    fn get_attribute_formats() -> Vec<wgpu::VertexFormat> {
        vec![wgpu::VertexFormat::Float3, wgpu::VertexFormat::Float3]
    }
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

/*--------------------------------------------------------------------------------------------------*/

struct GameApplication {}

impl Application<MainScene> for GameApplication {
    fn create_scene(&mut self, renderer: &mut Renderer) -> MainScene {
        MainScene::new(renderer)
    }
}

struct MainScene {
    previous_mouse_input: Option<winit::dpi::PhysicalPosition<f64>>,
    angle_multiplier: f32,
    movement_speed: f32,

    camera: CameraHandle,
    light: LightHandle,
    object_handle: InstancedObjectHandle,

    light_object_handle: ObjectHandle,
}

impl Scene for MainScene {
    fn new(renderer: &mut Renderer) -> Self {
        let window_size = renderer.get_window_size();
        let mut camera = CameraHandle::new(&renderer, 0, wgpu::ShaderStage::VERTEX);
        camera
            .set_perspective(75.0, window_size.width as f32 / window_size.height as f32)
            .look_at(
                cgmath::Point3 {
                    x: 1.0,
                    y: 6.0,
                    z: -1.0,
                },
                cgmath::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            );

        let light_color = cgmath::Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let light_position = cgmath::Vector3 {
            x: -1.0,
            y: -0.3,
            z: 0.0,
        };

        let mut light = LightHandle::new(&renderer, 2, wgpu::ShaderStage::FRAGMENT);
        light
            .set_color(light_color.clone())
            .set_position(light_position.clone());

        let n_instances = 3;
        let mut object_handle =
            InstancedObjectHandle::new(&renderer, 1, wgpu::ShaderStage::VERTEX, n_instances);
        object_handle
            .get_object(0)
            .set_color(0.9, 0.2, 0.2)
            .translate(2.0, 0.0, 0.0);
        object_handle
            .get_object(1)
            .set_color(0.2, 0.9, 0.2)
            .translate(-1.0, 1.0, 0.0);

        object_handle
            .get_object(2)
            .set_color(0.2, 0.2, 0.9)
            .translate(1.0, -0.5, -3.0);

        let mut rpass = RenderPass::new(
            AttachmentView::Dynamic,
            wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: true,
            },
        );

        let depth_texture_view = renderer.create_depth_texture_view();
        rpass.add_depth_attachment(
            depth_texture_view,
            wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: false,
            },
        );



        // ADD ENTRIES
        let (vertices, indices) = create_vertices();
        let material_pipeline = renderer.create_pipeline::<VertexBasic>(
            "examples/basic/shaders/material.vert",
            "examples/basic/shaders/material.frag",
            BindingEntries::new()
                .add(camera.get_binding_layout())
                .add(object_handle.get_binding_layout())
                .add(light.get_binding_layout()),
            &vec![EntityDescriptor {
                vertices,
                indices,
                bindings: vec![
                    camera.get_binding(),
                    object_handle.get_binding(),
                    light.get_binding(),
                ],
                n_instances,
            }],
        );
        rpass.add_pipeline(material_pipeline);

        let mut light_object_handle = ObjectHandle::new(&renderer, 1, wgpu::ShaderStage::VERTEX);
        light_object_handle
            .get_object()
            .set_color(light_color.x, light_color.y, light_color.z)
            .translate(light_position.x, light_position.y, light_position.z)
            .rescale(0.05, 0.05, 0.05);

        let (vertices, indices) = create_vertices();
        let light_pipeline = renderer.create_pipeline(
            "examples/basic/shaders/light.vert",
            "examples/basic/shaders/light.frag",
            BindingEntries::new()
                .add(camera.get_binding_layout())
                .add(light_object_handle.get_binding_layout()),
            &vec![EntityDescriptor {
                vertices,
                indices,
                bindings: vec![camera.get_binding(), light_object_handle.get_binding()],
                n_instances: 1,
            }],
        );
        rpass.add_pipeline(light_pipeline);

        renderer.add_render_pass(rpass);

        Self {
            previous_mouse_input: None,
            angle_multiplier: 0.004,
            movement_speed: 0.1,

            camera,
            light,
            object_handle,
            light_object_handle,
        }
    }

    fn game_loop(&mut self, input_state: &InputState, renderer: &mut Renderer) {
        match input_state.mouse.button {
            Some(_) => {
                if let Some(previous) = self.previous_mouse_input {
                    let delta_x = input_state.mouse.location.x - previous.x;
                    let delta_y = input_state.mouse.location.y - previous.y;

                    self.camera.rotate_around_center(
                        -self.angle_multiplier * delta_x as f32,
                        -self.angle_multiplier * delta_y as f32,
                    );
                }

                self.previous_mouse_input = Some(input_state.mouse.location);
            }
            None => self.previous_mouse_input = None,
        }

        {
            let keyboard_pressed = &input_state.keyboard.pressed;

            let direction = self.camera.get_direction().normalize();
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
                movement = movement.normalize_to(self.movement_speed);
            }

            self.camera.translate(movement.x, movement.y, movement.z);
        }

        renderer.update_binding(&self.camera);
        renderer.update_binding(&self.light);
        renderer.update_binding(&self.object_handle);
        renderer.update_binding(&self.light_object_handle);
    }
}

/*--------------------------------------------------------------------------------------------------*/

fn main() {
    let mut app = GameApplication {};
    app.run();
}
