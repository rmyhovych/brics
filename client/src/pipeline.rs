use wgpu;

use crate::object_factory::ObjectLayout;

/*--------------------------------------------------------------------------------------------------*/

pub struct Builder<'a> {
    vertex_shader: Option<&'a wgpu::ShaderModule>,
    fragment_shader: Option<&'a wgpu::ShaderModule>,

    color_format: Option<wgpu::TextureFormat>,

    pipeline_layout: Option<&'a wgpu::PipelineLayout>,

    vertex_size: Option<u32>,
    vertex_attribute_descriptors: Option<&'a Vec<wgpu::VertexAttributeDescriptor>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder {
            vertex_shader: None,
            fragment_shader: None,

            color_format: None,

            pipeline_layout: None,

            vertex_size: None,
            vertex_attribute_descriptors: None,
        }
    }

    pub fn set_shaders(
        &mut self,
        vertex_module: &'a wgpu::ShaderModule,
        fragment_module: &'a wgpu::ShaderModule,
    ) -> &mut Self {
        self.vertex_shader = Some(vertex_module);
        self.fragment_shader = Some(fragment_module);

        self
    }

    pub fn set_object_layout<T>(&mut self, object_layout: &'a ObjectLayout<T>) -> &mut Self {
        self.pipeline_layout = Some(object_layout.get_pipeline_layout());

        self.vertex_size = Some(object_layout.get_vertex_size());
        self.vertex_attribute_descriptors = Some(object_layout.get_vertex_attribute_descriptors());

        self
    }

    pub fn set_color_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.color_format = Some(format);

        self
    }

    pub fn build(&self, device: &wgpu::Device) -> Pipeline {
        let vertex_buffer_descriptor = wgpu::VertexBufferDescriptor {
            stride: self.vertex_size.expect("Missing Vertex Size!") as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: self
                .vertex_attribute_descriptors
                .expect("Missing Vertex Attribute Descriptors!")
                .as_slice(),
        };
        Pipeline {
            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: self.pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: self.vertex_shader.expect("Missing Vertex Shader!"),
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: self
                        .fragment_shader
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
                    format: self.color_format.expect("Missing Color Format!"),
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
                    vertex_buffers: &[vertex_buffer_descriptor],
                },
                sample_count: 1,
                sample_mask: 0,
                alpha_to_coverage_enabled: false,
            }),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn apply_on_render_pass<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        _: &wgpu::Queue,
    ) {
        render_pass.set_pipeline(&self.pipeline);
    }
}
