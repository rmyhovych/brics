extern crate rustgame;

use rustgame::{
    application::Application,
    binding::{
        sampler::{SamplerAddressMode, SamplerBinding, SamplerBindingLayout, SamplerFilterMode},
        texture::{TextureBinding, TextureBindingLayout},
    },
    handle::{
        camera::CameraHandle,
        light::LightHandle,
        object::{InstancedObjectHandle, ObjectHandle},
        BindingHandle, BindingLayoutHandle,
    },
    input::InputState,
    pipeline::{BindingEntries, EntityDescriptor, Pipeline, Vertex},
    render_pass::{AttachmentView, RenderPass},
    renderer::Renderer,
    scene::Scene,
};

use cgmath::{self, InnerSpace};

use wgpu;
use winit::{self, event::VirtualKeyCode};

struct VertexBasic {
    _position: cgmath::Point3<f32>,
    _normal: cgmath::Vector3<f32>,
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
        _position: cgmath::Point3 {
            x: raw_vertex[0],
            y: raw_vertex[1],
            z: raw_vertex[2],
        },
        _normal: cgmath::Vector3 {
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
    light_camera: CameraHandle,

    light: LightHandle,
    cubes: InstancedObjectHandle,
}

impl Scene for MainScene {
    fn new(renderer: &mut Renderer) -> Self {
        let camera = Self::create_main_camera(renderer);
        let light_camera = Self::create_light_camera(renderer);
        let light = Self::create_light(renderer);
        let cubes = Self::create_main_object_handle(renderer);

        let depth_texture_layout = TextureBindingLayout::new_sampled_output(
            3,
            wgpu::ShaderStage::FRAGMENT,
            wgpu::Extent3d {
                width: 2048,
                height: 2048,
                depth: 1,
            },
            wgpu::TextureFormat::Depth32Float,
        );
        let depth_texture_binding = renderer.create_binding(&depth_texture_layout);

        let shadow_pipeline = Self::create_shadow_pipeline(renderer, &light_camera, &cubes);
        let material_pipeline = Self::create_material_pipeline(
            renderer,
            &camera,
            &cubes,
            &light,
            &light_camera,
            &depth_texture_layout,
            &depth_texture_binding,
        );

        let texture_view = depth_texture_binding.create_texture_view();
        Self::create_shadow_render_pass(renderer, shadow_pipeline, texture_view);

        Self::create_material_render_pass(renderer, material_pipeline);

        Self {
            previous_mouse_input: None,
            angle_multiplier: 0.004,
            movement_speed: 0.1,

            camera,
            light_camera,
            light,
            cubes,
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

        self.light_camera
            .look_at_dir(self.camera.get_center(), -self.light.get_direction());

        renderer.update_binding(&self.camera);
        renderer.update_binding(&self.light_camera);
        renderer.update_binding(&self.light);
        renderer.update_binding(&self.cubes);
    }
}

impl MainScene {
    fn create_main_camera(renderer: &Renderer) -> CameraHandle {
        let window_size = renderer.get_window_size();
        let mut camera = CameraHandle::new(&renderer, wgpu::ShaderStage::VERTEX);
        camera
            .set_perspective(75.0, window_size.width as f32 / window_size.height as f32)
            .look_at(
                cgmath::Point3 {
                    x: 1.0,
                    y: 10.0,
                    z: -1.0,
                },
                cgmath::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            );

        camera
    }

    fn create_light_camera(renderer: &Renderer) -> CameraHandle {
        let camera_cube_size = 20.0;
        let mut camera = CameraHandle::new(&renderer, wgpu::ShaderStage::VERTEX);
        camera
            .set_ortho(
                -camera_cube_size,
                camera_cube_size,
                -camera_cube_size,
                camera_cube_size,
                -2.0 * camera_cube_size,
                camera_cube_size,
            )
            .look_at(
                cgmath::Point3 {
                    x: 1.0,
                    y: 10.0,
                    z: -1.0,
                },
                cgmath::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            );

        camera
    }

    fn create_light(renderer: &Renderer) -> LightHandle {
        let light_color = cgmath::Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let light_direction = cgmath::Vector3 {
            x: -1.0,
            y: 1.5,
            z: 0.5,
        };
        let mut light = LightHandle::new(renderer, wgpu::ShaderStage::FRAGMENT);
        light
            .set_color(light_color.clone())
            .set_direction(light_direction);

        light
    }

    fn create_main_object_handle(renderer: &Renderer) -> InstancedObjectHandle {
        let n_instances = 3;
        let mut object_handle =
            InstancedObjectHandle::new(&renderer, wgpu::ShaderStage::VERTEX, n_instances);
        object_handle
            .get_object(0)
            .set_color(0.8, 0.8, 0.9)
            .translate(0.0, -1.0, 0.0)
            .rescale(8.0, 0.1, 8.0);
        object_handle
            .get_object(1)
            .set_color(0.2, 0.9, 0.2)
            .translate(-1.0, 1.0, 0.0);

        object_handle
            .get_object(2)
            .set_color(0.2, 0.2, 0.9)
            .translate(1.0, 0.05, -3.0);

        object_handle
    }

    fn create_shadow_pipeline(
        renderer: &Renderer,
        light_camera: &CameraHandle,
        cubes: &InstancedObjectHandle,
    ) -> Pipeline {
        let (vertices, indices) = create_vertices();
        renderer.create_pipeline::<VertexBasic>(
            "examples/basic/shaders/shadow.vert",
            "examples/basic/shaders/shadow.frag",
            BindingEntries::new()
                .add(light_camera.get_binding_layout())
                .add(cubes.get_binding_layout()),
            None,
            Some(wgpu::DepthStencilStateDescriptor {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 2, // corresponds to bilinear filtering
                depth_bias_slope_scale: 2.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            &vec![EntityDescriptor {
                vertices,
                indices,
                bindings: vec![light_camera.get_binding(), cubes.get_binding()],
                n_instances: cubes.get_n_instances(),
            }],
        )
    }

    fn create_material_pipeline(
        renderer: &Renderer,
        camera: &CameraHandle,
        cubes: &InstancedObjectHandle,
        light: &LightHandle,
        light_camera: &CameraHandle,

        shadow_texture_layout: &TextureBindingLayout,
        shadow_texture: &TextureBinding,
    ) -> Pipeline {
        let sampler_layout = SamplerBindingLayout::new(
            4,
            wgpu::ShaderStage::FRAGMENT,
            SamplerAddressMode {
                u: wgpu::AddressMode::ClampToEdge,
                v: wgpu::AddressMode::ClampToEdge,
                w: wgpu::AddressMode::ClampToEdge,
            },
            SamplerFilterMode {
                mag: wgpu::FilterMode::Linear,
                min: wgpu::FilterMode::Linear,
                mipmap: wgpu::FilterMode::Nearest,
            },
            Some(wgpu::CompareFunction::LessEqual),
        );
        let sampler_binding = renderer.create_binding(&sampler_layout);

        let (vertices, indices) = create_vertices();

        renderer.create_pipeline::<VertexBasic>(
            "examples/basic/shaders/material.vert",
            "examples/basic/shaders/material.frag",
            BindingEntries::new()
                .add(camera.get_binding_layout())
                .add(cubes.get_binding_layout())
                .add(light.get_binding_layout())
                .add(light_camera.get_binding_layout())
                .add(shadow_texture_layout)
                .add(&sampler_layout),
            Some(wgpu::ColorStateDescriptor {
                format: Renderer::get_swapchain_color_format(),
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }),
            Some(wgpu::DepthStencilStateDescriptor {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            }),
            &vec![EntityDescriptor {
                vertices,
                indices,
                bindings: vec![
                    camera.get_binding(),
                    cubes.get_binding(),
                    light.get_binding(),
                    light_camera.get_binding(),
                    shadow_texture,
                    &sampler_binding,
                ],
                n_instances: cubes.get_n_instances(),
            }],
        )
    }

    fn create_shadow_render_pass(
        renderer: &mut Renderer,
        shadow_pipeline: Pipeline,
        depth_output: wgpu::TextureView,
    ) {
        let mut rpass = RenderPass::new();
        rpass.set_depth_attachment(
            depth_output,
            wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            },
        );

        rpass.add_pipeline(shadow_pipeline);
        renderer.add_render_pass(rpass);
    }

    fn create_material_render_pass(renderer: &mut Renderer, material_pipeline: Pipeline) {
        let depth_texture_view = renderer.create_depth_texture_view();
        let mut rpass = RenderPass::new();
        rpass
            .set_color_attachment(
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
            )
            .set_depth_attachment(
                depth_texture_view,
                wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: false,
                },
            );

        rpass.add_pipeline(material_pipeline);

        renderer.add_render_pass(rpass);
    }
}

/*--------------------------------------------------------------------------------------------------*/

fn main() {
    let mut app = GameApplication {};
    app.run();
}
