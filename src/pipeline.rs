use crate::{
    binding::{Binding, BindingLayout},
    handle::{BindingHandle, BindingHandleLayout},
};

pub struct Shaders {
    pub vertex_module: wgpu::ShaderModule,
    pub fragment_module: wgpu::ShaderModule,
}

/*--------------------------------------------------------------------------------------------------*/

pub trait Vertex {
    fn get_attribute_descriptors() -> Vec<wgpu::VertexAttributeDescriptor> {
        let mut vertex_attribute_descriptors = Vec::<wgpu::VertexAttributeDescriptor>::new();

        let mut shader_location: wgpu::ShaderLocation = 0;
        let mut offset: wgpu::BufferAddress = 0;
        for format in Self::get_attribute_formats().iter() {
            vertex_attribute_descriptors.push(wgpu::VertexAttributeDescriptor {
                format: *format,
                offset,
                shader_location,
            });

            shader_location += 1;
            offset += format.size();
        }

        vertex_attribute_descriptors
    }

    fn get_attribute_formats() -> Vec<wgpu::VertexFormat>;
}

/*--------------------------------------------------------------------------------------------------*/

pub struct BindingLayoutEntries {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindingLayoutEntries {
    pub fn new() -> BindingLayoutEntries {
        BindingLayoutEntries {
            entries: Vec::new(),
        }
    }

    pub fn add<A: Binding, B: BindingLayout<A>, H: BindingHandle>(
        mut self,
        handle_layout: &dyn BindingHandleLayout<A, B, H>,
    ) -> Self {
        let mut layout_entry: wgpu::BindGroupLayoutEntry =
            handle_layout.get_binding_layout().get_entry();
        layout_entry.binding = self.entries.len() as u32;

        self.entries.push(layout_entry);

        self
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Pipeline {
    handle: wgpu::RenderPipeline,

    bind_group_layout: wgpu::BindGroupLayout,
    entities: Vec<Entity>,
}

impl Pipeline {
    pub fn new<T: Vertex>(
        device: &wgpu::Device,
        shaders: &Shaders,
        binding_entries: &BindingLayoutEntries,

        color_state: Option<wgpu::ColorStateDescriptor>,
        depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
        rasterization_state: Option<wgpu::RasterizationStateDescriptor>,
    ) -> Pipeline {
        let attribute_descriptors = T::get_attribute_descriptors();
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: binding_entries.entries.as_slice(),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buffer_descriptor = wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: attribute_descriptors.as_slice(),
        };

        let color_states_vec: Vec<wgpu::ColorStateDescriptor> = match color_state {
            None => vec![],
            Some(desc) => vec![desc],
        };

        let handle = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &shaders.vertex_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &shaders.fragment_module,
                entry_point: "main",
            }),
            rasterization_state,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: color_states_vec.as_slice(),
            depth_stencil_state,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[vertex_buffer_descriptor],
            },
            sample_count: 1,
            sample_mask: 0,
            alpha_to_coverage_enabled: false,
        });

        Pipeline {
            handle,

            bind_group_layout,
            entities: Vec::new(),
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.handle);
        self.entities
            .iter()
            .for_each(|entity| entity.render(render_pass));
    }

    pub fn add_entity(
        &mut self,
        device: &wgpu::Device,

        geometry: &Geometry,
        handles: Vec<&dyn BindingHandle>,

        n_instances: u32,
    ) {
        let bind_group = self.create_bind_group(device, handles);

        self.entities.push(Entity {
            vertex_buffer: std::rc::Rc::clone(&geometry.vertex_buffer),
            index_buffer: std::rc::Rc::clone(&geometry.index_buffer),

            n_indices: geometry.n_indices,

            n_instances,

            bind_group,
        });
    }

    /*-------------------------------------------------*/

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        handles: Vec<&dyn BindingHandle>,
    ) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = handles
            .iter()
            .map(|handle| handle.get_binding().get_resource())
            .enumerate()
            .map(|(index, resource)| wgpu::BindGroupEntry {
                binding: index as u32,
                resource,
            })
            .collect();

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: entries.as_slice(),
        })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Geometry {
    vertex_buffer: std::rc::Rc<wgpu::Buffer>,
    index_buffer: std::rc::Rc<wgpu::Buffer>,

    n_indices: u32,
}

impl Geometry {
    pub fn new(vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, n_indices: u32) -> Self {
        Self {
            vertex_buffer: std::rc::Rc::new(vertex_buffer),
            index_buffer: std::rc::Rc::new(index_buffer),
            n_indices,
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

struct Entity {
    vertex_buffer: std::rc::Rc<wgpu::Buffer>,
    index_buffer: std::rc::Rc<wgpu::Buffer>,

    n_indices: u32,
    n_instances: u32,

    bind_group: wgpu::BindGroup,
}

impl Entity {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));

        render_pass.draw_indexed(0..self.n_indices, 0, 0..self.n_instances);
    }
}
