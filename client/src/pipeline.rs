use wgpu::{self, util::DeviceExt};

use crate::binding;

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

pub struct BindingEntries {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindingEntries {
    pub fn new() -> BindingEntries {
        BindingEntries {
            entries: Vec::new(),
        }
    }

    pub fn add<A: binding::Binding, B: binding::BindingLayout<A>>(
        mut self,
        binding_layout: &B,
    ) -> Self {
        let mut layout_entry: wgpu::BindGroupLayoutEntry = binding_layout.get_entry();
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
        binding_entries: &BindingEntries,

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

    pub fn add_entity<T: Vertex>(
        &mut self,
        device: &wgpu::Device,

        descriptor: &EntityDescriptor<T>,
    ) {
        let vertex_buffer =
            self.create_device_buffer(device, &descriptor.vertices, wgpu::BufferUsage::VERTEX);
        let index_buffer =
            self.create_device_buffer(device, &descriptor.indices, wgpu::BufferUsage::INDEX);

        let bind_group = self.create_bind_group(device, &descriptor.bindings);

        self.entities.push(Entity {
            vertex_buffer,
            index_buffer,

            n_indices: descriptor.indices.len() as u32,
            n_instances: descriptor.n_instances,

            bind_group,
        });
    }

    /*-------------------------------------------------*/

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        bindings: &Vec<&dyn binding::Binding>,
    ) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = bindings
            .iter()
            .map(|binding| binding.get_resource())
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

    fn create_device_buffer<K>(
        &self,
        device: &wgpu::Device,
        contents: &Vec<K>,
        usage: wgpu::BufferUsage,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe {
                std::slice::from_raw_parts(
                    contents.as_slice().as_ptr() as *const u8,
                    std::mem::size_of::<K>() * contents.len(),
                )
            },
            usage,
        })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct EntityDescriptor<'a, T: Vertex> {
    pub vertices: Vec<T>,
    pub indices: Vec<u16>,
    pub bindings: Vec<&'a dyn binding::Binding>,

    pub n_instances: u32,
}

struct Entity {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

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
