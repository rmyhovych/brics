use super::vertex::VertexBasic;

use brics::{
    application::{Application, Visual},
    binding::{
        sampler::{SamplerAddressMode, SamplerFilterMode},
        texture::TextureBinding,
    },
    graphics::GraphicsManager,
    handle::{
        camera::{CameraHandle, CameraHandleLayout},
        light::{LightHandle, LightHandleLayout},
        sampler::{SamplerHandle, SamplerHandleLayout},
        shape::{ShapeHandle, ShapeHandleLayout},
        texture::{TextureHandle, TextureHandleLayout},
        BindingHandle, BindingHandleLayout, BindingProxy,
    },
    input::InputState,
    pipeline::{BindingLayoutEntries, Geometry, Pipeline, Vertex},
    render_pass::{AttachmentView, RenderPass},
    renderer::Renderer,
};

use wgpu;
use winit;

pub struct BasicVisual {
    graphics: GraphicsManager,
    renderer: Renderer,

    pipeline_id: (u32, u32),
    shadow_pipeline_id: (u32, u32),

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

impl Visual for BasicVisual {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let graphics: GraphicsManager =
            futures::executor::block_on(GraphicsManager::new(event_loop));

        let camera_handle_layout = CameraHandleLayout::new(wgpu::ShaderStage::VERTEX);
        let camera = Self::create_main_camera(&camera_handle_layout, &graphics);

        let light_camera = Self::create_light_camera(&camera_handle_layout, &graphics);

        let light_handle_layout = LightHandleLayout::new(wgpu::ShaderStage::FRAGMENT);
        let light = Self::create_light(&light_handle_layout, &graphics);

        let shape_handle_layout = ShapeHandleLayout::new(wgpu::ShaderStage::VERTEX);

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

        let mut renderer = Renderer::new();
        let shadow_pipeline_id = Self::create_shadow_render_pass(
            &graphics,
            &mut renderer,
            shadow_pipeline,
            texture_view,
        );
        let pipeline_id =
            Self::create_material_render_pass(&graphics, &mut renderer, material_pipeline);

        Self {
            graphics,
            renderer,

            pipeline_id,
            shadow_pipeline_id,

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

    fn render(&mut self) {
        self.update_bindings();
        self.graphics.render(&self.renderer);
    }

    fn request_redraw(&self) {
        self.graphics.request_redraw();
    }
}

impl BasicVisual {
    pub fn create_geometry(&self, vertices: Vec<impl Vertex>, indices: Vec<u16>) -> Geometry {
        self.graphics.create_geometry(vertices, indices)
    }

    pub fn get_main_camera(&mut self) -> BindingProxy<CameraHandle> {
        BindingProxy::new(&mut self.camera)
    }

    pub fn get_light_camera(&mut self) -> BindingProxy<CameraHandle> {
        BindingProxy::new(&mut self.light_camera)
    }

    pub fn get_light(&mut self) -> BindingProxy<LightHandle> {
        BindingProxy::new(&mut self.light)
    }

    pub fn create_shape_entity(&mut self, geometry: &Geometry) -> BindingProxy<ShapeHandle> {
        let pipeline = self
            .renderer
            .get_render_pass(self.pipeline_id.0)
            .get_pipeline(self.pipeline_id.1);

        let shape = self.shape_handle_layout.create_handle(&self.graphics);
        self.shapes.push(shape);
        let shape_ref = self.shapes.last_mut().unwrap();

        self.graphics.add_pipeline_entity(
            pipeline,
            geometry,
            vec![
                &self.camera,
                shape_ref,
                &self.light,
                &self.light_camera,
                &self.depth_texture_handle,
                &self.depth_sampler_handle,
            ],
        );

        let pipeline = self
            .renderer
            .get_render_pass(self.shadow_pipeline_id.0)
            .get_pipeline(self.shadow_pipeline_id.1);

        self.graphics
            .add_pipeline_entity(pipeline, geometry, vec![&self.light_camera, shape_ref]);

        BindingProxy::new(shape_ref)
    }

    /*------------------------------------------------------------*/

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
        graphics: &GraphicsManager,
        renderer: &mut Renderer,
        shadow_pipeline: Pipeline,
        depth_output: wgpu::TextureView,
    ) -> (u32, u32) {
        let mut rpass = RenderPass::new();
        rpass.set_depth_attachment(
            depth_output,
            wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            },
        );

        let mut pipeline_ids: (u32, u32) = (0, 0);
        pipeline_ids.1 = rpass.add_pipeline(shadow_pipeline);
        pipeline_ids.0 = renderer.add_render_pass(rpass);
        pipeline_ids
    }

    fn create_material_render_pass(
        graphics: &GraphicsManager,
        renderer: &mut Renderer,
        material_pipeline: Pipeline,
    ) -> (u32, u32) {
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

        let mut pipeline_ids: (u32, u32) = (0, 0);
        pipeline_ids.1 = rpass.add_pipeline(material_pipeline);
        pipeline_ids.0 = renderer.add_render_pass(rpass);

        pipeline_ids
    }

    fn update_bindings(&self) {
        self.graphics.update_handle(&self.camera);
        self.graphics.update_handle(&self.light_camera);

        self.graphics.update_handle(&self.light);
        self.shapes
            .iter()
            .for_each(|handle| self.graphics.update_handle(handle));
    }
}
