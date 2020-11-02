use bytemuck;
use cgmath::{Matrix4, Quaternion, Vector3};
use std::mem;
use wgpu::{self, util::DeviceExt};

/*--------------------------------------------------------------------------------------------------*/

pub struct Object {
    translation: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,

    color: Vector3<f32>,
}

impl Object {
    fn new() -> Object {
        Object {
            translation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quaternion::from_sv(0.0, Vector3::unit_y()),
            scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Vector3 { x: r, y: g, z: b }
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = Vector3 { x, y, z };
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.translation += Vector3 { x, y, z };
    }

    pub fn rotate(&mut self, axis: &Vector3<f32>, angle: f32) {
        let delta_rotation = Quaternion::from_sv(angle, *axis);
        self.rotation = delta_rotation * self.rotation;
    }

    fn get_model_mat(&self) -> Matrix4<f32> {
        let mut model = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        model = Matrix4::from(self.rotation) * model;
        model = Matrix4::from_translation(self.translation);

        model
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectFamily {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    model_uniform_buffer: wgpu::Buffer,

    objects: Vec<Object>,
}

impl ObjectFamily {
    pub fn new(
        device: &wgpu::Device,
        vertex_data: &Vec<[f32; 3]>,
        index_data: &Vec<u16>,
    ) -> ObjectFamily {
        ObjectFamily {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex_data),
                usage: wgpu::BufferUsage::VERTEX,
            }),

            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(index_data),
                usage: wgpu::BufferUsage::INDEX,
            }),

            model_uniform_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Model Uniform Buffer"),
                size: ObjectFamily::get_uniform_size(),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            }),

            objects: Vec::new(),
        }
    }

    pub fn create_object(&mut self) -> &mut Object {
        self.objects.push(Object::new());
        self.objects.last_mut().unwrap()
    }

    pub fn apply_on_renderpass(&self, renderpass: &wgpu::RenderPass, write_queue: &wgpu::Queue) {
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    }

    pub fn get_bind_group_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: wgpu::BufferSize::new(ObjectFamily::get_uniform_size()),
            },
            count: None,
        }
    }

    pub fn get_bind_group_entry(&self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(self.model_uniform_buffer.slice(..)),
        }
    }

    fn get_uniform_size() -> u64 {
        mem::size_of::<Matrix4<f32>>() as u64
    }
}
