use bytemuck;
use wgpu;

use crate::binding;

use std::iter::Iterator;

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectLayoutBuilder<'a, T> {
    device: &'a wgpu::Device,
    binding_layouts: Vec<&'a dyn binding::BindingLayout>,
    attribute_formats: Vec<wgpu::VertexFormat>,
}

impl<'a, T> ObjectLayoutBuilder<'a, T> {
    pub fn new(device: &wgpu::Device) -> Self {
        ObjectLayout {
            device,
            binding_layouts: Vec::new(),
            attribute_formats: Vec::new(),
        }
    }

    pub fn add_binding_layout(&mut self, binding: &dyn binding::BindingLayout) -> &mut Self {
        self.binding_layouts.push(binding);
        self
    }

    pub fn push_attribute_format(&mut self, format: wgpu::VertexFormat) -> &mut Self {
        self.attribute_formats.push(format);
        self
    }

    pub fn build(&self) -> ObjectLayout<T> {
        let vertex_attribute_descriptors = Vec::<wgpu::VertexAttributeDescriptor>::new();

        let mut shader_location: wgpu::ShaderLocation = 0;
        let mut offset: wgpu::BufferAddress = 0;
        for format in self.attribute_formats {
            vertex_attribute_descriptors.push(wgpu::VertexAttributeDescriptor {
                format,
                offset,
                shader_location,
            });

            shader_location += 1;
            offset += format.size();
        }

        ObjectLayout {
            device: self.device,
            bind_group_layout: self.create_bind_group_layout(),
            vertex_attribute_descriptors,
        }
    }

    fn create_bind_group_layout(&self) -> wgpu::BindGroupLayout {
        let mut entries: Vec<wgpu::BindGroupLayoutEntry> = Vec::new();
        for binding_layout in self.binding_layouts {
            entries.push(*binding_layout.get_layout_entry());
        }

        self.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: entries.as_slice(),
            })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectLayout<'a, T> {
    device: &'a wgpu::Device,
    bind_group_layout: wgpu::BindGroupLayout,
    vertex_attribute_descriptors: Vec<wgpu::VertexAttributeDescriptor>,
}

impl<'a, T> ObjectLayout<'a, T> {
    pub fn create_object(
        &self,
        vertices: &Vec<T>,
        indices: &Vec<u16>,
        bindings: &Vec<&dyn binding::Binding>,
    ) -> Object {
        self.create_object_instanced(vertices, indices, bindings, 1)
    }

    pub fn create_object_instanced(
        &self,
        vertices: &Vec<T>,
        indices: &Vec<u16>,
        bindings: &Vec<&dyn binding::Binding>,
        n_instances: u32,
    ) -> Object {
        let vertex_buffer = self.create_device_buffer(vertices, wgpu::BufferUsage::VERTEX);
        let index_buffer = self.create_device_buffer(indices, wgpu::BufferUsage::INDEX);

        let bind_group = self.create_bind_group(bindings);

        Object {
            vertex_buffer,
            index_buffer,

            n_indices: indices.len() as u32,
            n_instances,

            bind_group,
        }
    }

    pub fn get_vertex_buffer_descriptor(&self) -> wgpu::VertexBufferDescriptor {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<T>(),
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: self.vertex_attribute_descriptors.as_slice(),
        }
    }

    pub fn create_pipeline_layout(&self) -> wgpu::PipelineLayout {
        self.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&self.bind_group_layout],
                push_constant_ranges: &[],
            })
    }

    fn create_bind_group(&self, bindings: &Vec<&dyn binding::Binding>) -> wgpu::BindGroup {
        let entries: &[wgpu::BindingResource] = bindings
            .iter()
            .map(|binding: &dyn binding::Binding| *binding.get_resource())
            .collect()
            .as_slice();

        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries,
        })
    }

    fn create_device_buffer<K>(&self, data: Vec<K>, usage: wgpu::BufferUsage) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data.as_slice()),
                usage,
            })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct Object {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    n_indices: u32,
    n_instances: u32,

    bind_group: wgpu::BindGroup,
}

impl Object<'_> {
    fn apply_on_render_pass<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        write_queue: &wgpu::Queue,
    ) {
        render_pass.set_bind_group(0, self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.n_indices, 0, 0..self.n_instances);
    }
}
