use wgpu;

use crate::object_factory::ObjectLayout;

/*--------------------------------------------------------------------------------------------------*/

pub struct Builder<'a> {
    device: &'a wgpu::Device,

    vertex_shader: Option<wgpu::ShaderModule>,
    fragment_shader: Option<wgpu::ShaderModule>,

    pipeline_layout: Option<wgpu::PipelineLayout>,

    color_format: Option<wgpu::TextureFormat>,

    vertex_buffer_descriptor: Option<wgpu::VertexBufferDescriptor<'a>>,
}

impl Builder<'_> {
    pub fn new(device: &wgpu::Device) -> Builder {
        Builder {
            device,

            vertex_shader: None,
            fragment_shader: None,

            pipeline_layout: None,

            color_format: None,

            vertex_buffer_descriptor: None,
        }
    }

    pub fn set_shaders(&mut self, vertex_data: &[u8], fragment_data: &[u8]) -> &mut Builder {
        self.vertex_shader = Some(self.create_shader_module(vertex_data));
        self.fragment_shader = Some(self.create_shader_module(fragment_data));

        self
    }

    pub fn set_object_layout(&mut self, object_layout: &ObjectLayout) -> &mut Builder {
        self.pipeline_layout = Some(object_layout.create_pipeline_layout());
        self.vertex_buffer_descriptor = Some(object_layout.get_vertex_buffer_descriptor());

        self
    }

    pub fn set_color_format(&mut self, format: wgpu::TextureFormat) -> &mut Builder {
        self.color_format = Some(format);

        self
    }

    pub fn build<T>(&self) -> Pipeline {
        Pipeline {
            pipeline: self
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(
                        self.pipeline_layout
                            .as_mut()
                            .expect("Missing Object Layout!"),
                    ),
                    vertex_stage: wgpu::ProgrammableStageDescriptor {
                        module: &self.vertex_shader.as_mut().expect("Missing Vertex Shader!"),
                        entry_point: "main",
                    },
                    fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                        module: &self
                            .fragment_shader
                            .as_mut()
                            .expect("Missing Fragment Shader!"),
                        entry_point: "main",
                    }),
                    rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: wgpu::CullMode::Back,
                        ..Default::default()
                    }),
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    color_states: &[wgpu::ColorStateDescriptor {
                        format: self.color_format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilStateDescriptor::default(),
                    }),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[self
                            .vertex_buffer_descriptor
                            .expect("Missing Vertex Buffer Descriptor!")],
                    },
                    sample_count: 1,
                    sample_mask: 0,
                    alpha_to_coverage_enabled: false,
                }),
        }
    }

    fn create_shader_module(&self, spv_data: &[u8]) {
        self.device
            .create_shader_module(wgpu::ShaderModuleSource::SpirV(std::borrow::Cow::from(
                spv_data[..] as &[u32],
            )));
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    fn apply_on_render_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, _: &wgpu::Queue) {
        render_pass.set_pipeline(&self.pipeline);
    }
}
