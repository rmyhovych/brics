extern crate rustgame;

use rustgame::{
    application::Application,
    binding::{
        sampler::{SamplerAddressMode, SamplerFilterMode},
        texture::TextureBinding,
    },
    graphics::GraphicsManager,
    handle::{
        camera::{CameraHandle, CameraHandleLayout},
        light::{LightHandle, LightHandleLayout},
        sampler::{SamplerHandle, SamplerHandleLayout},
        shape::{ShapeHandle, ShapeHandleLayout, ShapeState},
        texture::{TextureHandle, TextureHandleLayout},
        BindingHandle, BindingHandleLayout,
    },
    input::InputState,
    logic::GameLogic,
    object,
    pipeline::{BindingLayoutEntries, Pipeline, Vertex},
    render_pass::{AttachmentView, RenderPass},
    renderer::Renderer,
};

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

use wgpu;
use winit::{self, event::VirtualKeyCode};

struct VertexBasic {
    _position: Point3<f32>,
    _normal: Vector3<f32>,
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

impl<'a> Application<MainRenderer, MainLogic> for GameApplication {
    fn create_renderer(event_loop: &winit::event_loop::EventLoop<()>) -> MainRenderer {
        MainRenderer::new(event_loop)
    }

    fn create_game_logic(renderer: &mut MainRenderer) -> MainLogic {
        MainLogic::new(renderer)
    }
}

struct MainRenderer {
    graphics: GraphicsManager,

    /*------------------*/
    light_handle_layout: LightHandleLayout,
    shape_handle_layout: ShapeHandleLayout,

    /*------------------*/
    depth_texture_handle: TextureHandle,
    depth_sampler_handle: SamplerHandle,
    light_camera: CameraHandle,

    /*------------------*/
    camera: CameraHandle,
    light: LightHandle,
    shapes: Vec<ShapeHandle>,
}

impl Renderer for MainRenderer {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let mut graphics: GraphicsManager =
            futures::executor::block_on(GraphicsManager::new(event_loop));

        let camera_handle_layout = CameraHandleLayout::new(wgpu::ShaderStage::VERTEX);
        let camera = Self::create_main_camera(&camera_handle_layout, &graphics);

        let light_camera = Self::create_light_camera(&camera_handle_layout, &graphics);

        let light_handle_layout = LightHandleLayout::new(wgpu::ShaderStage::FRAGMENT);
        let light = Self::create_light(&light_handle_layout, &graphics);

        let shape_handle_layout = ShapeHandleLayout::new(wgpu::ShaderStage::VERTEX);

        /*
        let (vertices, indices) = create_vertices();
        let cube_geometry = graphics.create_geometry(vertices, indices);
        */

        let (
            depth_texture_handle_layout,
            sampler_handle_layout,
            depth_texture_handle,
            depth_sampler_handle,
        ) = Self::create_depth_texture(&graphics);

        let shadow_pipeline = Self::create_shadow_pipeline(
            &graphics,
            BindingLayoutEntries::new()
                .add(&camera_handle_layout)
                .add(&shape_handle_layout),
        );

        let material_pipeline = Self::create_material_pipeline(
            &graphics,
            BindingLayoutEntries::new()
                .add(&camera_handle_layout)
                .add(&shape_handle_layout)
                .add(&light_handle_layout)
                .add(&camera_handle_layout)
                .add(&depth_texture_handle_layout)
                .add(&sampler_handle_layout),
        );

        let texture_view = depth_texture_handle.create_texture_view();
        // Self::create_shadow_render_pass(graphics, shadow_pipeline, texture_view);
        Self::create_material_render_pass(&mut graphics, material_pipeline);

        Self {
            graphics,

            light_handle_layout,
            shape_handle_layout,

            depth_texture_handle,
            depth_sampler_handle,
            light_camera,

            camera,
            light,
            shapes: Vec::new(),
        }
    }

    fn render(&mut self, input_state: &InputState) {
        self.update_bindings();
        self.graphics.render();
    }

    fn request_redraw(&self) {
        self.graphics.request_redraw();
    }
}

impl MainRenderer {
    fn create_main_camera(
        handle_layout: &CameraHandleLayout,
        graphics: &GraphicsManager,
    ) -> CameraHandle {
        let window_size = graphics.get_window_size();
        let mut camera = handle_layout.create_handle(graphics);
        camera
            .set_perspective(75.0, window_size.width as f32 / window_size.height as f32)
            .look_at(
                cgmath::Point3 {
                    x: 0.0,
                    y: 10.0,
                    z: 1.0,
                },
                cgmath::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            );

        camera
    }

    fn create_light_camera(
        handle_layout: &CameraHandleLayout,
        graphics: &GraphicsManager,
    ) -> CameraHandle {
        let camera_cube_size = 20.0;
        let mut camera = handle_layout.create_handle(graphics);
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

    fn create_light(handle_layout: &LightHandleLayout, graphics: &GraphicsManager) -> LightHandle {
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
        let mut light = handle_layout.create_handle(graphics);
        light
            .set_color(light_color.clone())
            .set_direction(light_direction);

        light
    }

    fn create_depth_texture(
        graphics: &GraphicsManager,
    ) -> (
        TextureHandleLayout,
        SamplerHandleLayout,
        TextureHandle,
        SamplerHandle,
    ) {
        let depth_texture_handle_layout = TextureHandleLayout::new(
            wgpu::ShaderStage::FRAGMENT,
            wgpu::Extent3d {
                width: 2048,
                height: 2048,
                depth: 1,
            },
            wgpu::TextureFormat::Depth32Float,
        );
        let depth_texture_handle = depth_texture_handle_layout.create_handle(graphics);

        let sampler_handle_layout = SamplerHandleLayout::new(
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
        let sampler_handle = sampler_handle_layout.create_handle(graphics);

        (
            depth_texture_handle_layout,
            sampler_handle_layout,
            depth_texture_handle,
            sampler_handle,
        )
    }

    fn create_shadow_pipeline(
        graphics: &GraphicsManager,
        entries: BindingLayoutEntries,
    ) -> Pipeline {
        graphics.create_pipeline::<VertexBasic>(
            "examples/basic/shaders/shadow.vert",
            "examples/basic/shaders/shadow.frag",
            entries,
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
                depth_bias: 2,
                depth_bias_slope_scale: 2.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
        )
    }

    fn create_material_pipeline(
        graphics: &GraphicsManager,
        entries: BindingLayoutEntries,
    ) -> Pipeline {
        graphics.create_pipeline::<VertexBasic>(
            "examples/basic/shaders/material.vert",
            "examples/basic/shaders/material.frag",
            entries,
            Some(wgpu::ColorStateDescriptor {
                format: GraphicsManager::get_swapchain_color_format(),
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
        )
    }

    fn create_shadow_render_pass(
        graphics: &mut GraphicsManager,
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
        graphics.add_render_pass(rpass);
    }

    fn create_material_render_pass(graphics: &mut GraphicsManager, material_pipeline: Pipeline) {
        let depth_texture_view = graphics.create_depth_texture_view();
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

        graphics.add_render_pass(rpass);
    }

    fn update_bindings(&self) {
        /*
        self.graphics.update_handle(self.camera.borrow());
        self.graphics.update_handle(&self.light_camera);

        self.graphics.update_handle(&self.light);
        self.shapes
            .iter()
            .for_each(|handle| self.graphics.update_handle(handle));
        */
    }
}

/*--------------------------------------------------------------------------------------------------*/

struct MainLogic {
    objects: Vec<object::Object>,
    controllers: Vec<Box<dyn Fn(&mut object::Object)>>,
}

impl GameLogic<MainRenderer> for MainLogic {
    fn new(renderer: &mut MainRenderer) -> Self {
        Self {
            objects: Vec::new(),
            controllers: Vec::new(),
        }
    }

    fn step(&mut self) {
        self.controllers
            .iter()
            .zip(self.objects.iter_mut())
            .for_each(|(controller, object)| {
                controller(object);
                object.apply_changes();
            });
    }
}

/*--------------------------------------------------------------------------------------------------*/

fn main() {
    let mut app = GameApplication {};
    app.run();
}
