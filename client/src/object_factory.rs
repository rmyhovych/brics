use wgpu::{self, util::DeviceExt};

use crate::binding;

use std::iter::Iterator;

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectLayoutBuilder {
    binding_layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    attribute_formats: Vec<wgpu::VertexFormat>,
}

impl ObjectLayoutBuilder {
    pub fn new() -> ObjectLayoutBuilder {
        ObjectLayoutBuilder {
            binding_layout_entries: Vec::new(),
            attribute_formats: Vec::new(),
        }
    }

    pub fn add_binding_layout<A: binding::Binding, B: binding::BindingLayout<A>>(
        &mut self,
        binding_layout: &B,
    ) -> &mut Self {
        self.binding_layout_entries.push(binding_layout.get_entry());

        self
    }

    pub fn push_attribute_format(&mut self, format: wgpu::VertexFormat) -> &mut Self {
        self.attribute_formats.push(format);
        self
    }

    pub fn build<T>(&self, device: &wgpu::Device) -> ObjectLayout<T> {
        let mut vertex_attribute_descriptors = Vec::<wgpu::VertexAttributeDescriptor>::new();

        let mut shader_location: wgpu::ShaderLocation = 0;
        let mut offset: wgpu::BufferAddress = 0;
        for format in self.attribute_formats.iter() {
            vertex_attribute_descriptors.push(wgpu::VertexAttributeDescriptor {
                format: *format,
                offset,
                shader_location,
            });

            shader_location += 1;
            offset += format.size();
        }

        let bind_group_layout = self.create_bind_group_layout(device);
        let pipeline_layout = self.create_pipeline_layout(device, &bind_group_layout);

        ObjectLayout::<T> {
            bind_group_layout,
            pipeline_layout,

            vertex_attribute_descriptors,
            __phantom: std::marker::PhantomData,
        }
    }

    fn create_bind_group_layout(&self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: self.binding_layout_entries.as_slice(),
        })
    }

    fn create_pipeline_layout(
        &self,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        })
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectLayout<T> {
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,

    vertex_attribute_descriptors: Vec<wgpu::VertexAttributeDescriptor>,
    __phantom: std::marker::PhantomData<T>,
}

impl<T> ObjectLayout<T> {
    pub fn create_object(
        &self,
        device: &wgpu::Device,

        vertices: &Vec<T>,
        indices: &Vec<u16>,
        bindings: &Vec<&dyn binding::Binding>,
    ) -> Object {
        self.create_object_instanced(device, vertices, indices, bindings, 1)
    }

    pub fn create_object_instanced(
        &self,
        device: &wgpu::Device,

        vertices: &Vec<T>,
        indices: &Vec<u16>,
        bindings: &Vec<&dyn binding::Binding>,
        n_instances: u32,
    ) -> Object {
        let vertex_buffer = self.create_device_buffer(device, vertices, wgpu::BufferUsage::VERTEX);
        let index_buffer = self.create_device_buffer(device, indices, wgpu::BufferUsage::INDEX);

        let bind_group = self.create_bind_group(device, bindings);

        Object {
            vertex_buffer,
            index_buffer,

            n_indices: indices.len() as u32,
            n_instances,

            bind_group,
        }
    }

    pub fn get_vertex_size(&self) -> u32 {
        std::mem::size_of::<T>() as u32
    }

    pub fn get_pipeline_layout(&self) -> &wgpu::PipelineLayout {
        &self.pipeline_layout
    }

    pub fn get_vertex_attribute_descriptors(&self) -> &Vec<wgpu::VertexAttributeDescriptor> {
        &self.vertex_attribute_descriptors
    }

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
            .collect::<Vec<wgpu::BindGroupEntry>>();

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

#[derive(Debug)]
pub struct Object {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    n_indices: u32,
    n_instances: u32,

    bind_group: wgpu::BindGroup,
}

impl Object {
    pub fn apply_on_render_pass<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        _: &wgpu::Queue,
    ) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));

        render_pass.draw_indexed(0..self.n_indices, 0, 0..self.n_instances);
    }
}
