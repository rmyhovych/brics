use crate::layout::{Uniform, LayoutHandler};
use bytemuck;
use cgmath::{InnerSpace, Matrix4, Quaternion, Rotation3, Vector3};
use wgpu::{self, util::DeviceExt};

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
pub struct ObjectUniform {
    rotation: Matrix4<f32>,
    model: Matrix4<f32>,
    color: Vector3<f32>,
    _padding: f32, // needed for an alignment with glsl
}

impl Uniform for ObjectUniform {}

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
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
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

    pub fn rotate(&mut self, axis: &Vector3<f32>, angle: &cgmath::Rad<f32>) {
        let delta_rotation = Quaternion::from_axis_angle(*axis, *angle);
        self.rotation = (self.rotation * delta_rotation).normalize();
    }

    fn get_uniform_data(&self) -> ObjectUniform {
        let mut model = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        let rotation = Matrix4::from(self.rotation);
        model = rotation * model;
        model = Matrix4::from_translation(self.translation) * model;

        ObjectUniform {
            rotation,
            model,
            color: self.color,
            _padding: 0.0,
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct ObjectFamily {
    uniform_buffer: wgpu::Buffer,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    n_indexes: u32,

    objects: Vec<Object>,
}

impl ObjectFamily {
    pub fn new(
        device: &wgpu::Device,
        vertex_data: &Vec<[f32; 6]>,
        index_data: &Vec<u16>,
        n_objects: u64,
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

            n_indexes: index_data.len() as _,

            uniform_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Uniform Buffer"),
                size: n_objects * ObjectUniform::size(),
                usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            }),

            objects: (0..n_objects)
                .map(move |_| Object::new())
                .collect::<Vec<Object>>(),
        }
    }

    pub fn get(&mut self, index: usize) -> &mut Object {
        self.objects.get_mut(index).unwrap()
    }

    pub fn create_object(&mut self) -> &mut Object {
        self.objects.push(Object::new());
        self.objects.last_mut().unwrap()
    }
}

impl UniformDescriptor<ObjectUniform> for ObjectFamily {
    fn get_bind_group_layout_entry(&self) -> Vec<wgpu::BindGroupLayoutEntry> {
        vec![wgpu::BindGroupLayoutEntry {
            binding: self.get_binding(),
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::StorageBuffer {
                dynamic: false,
                readonly: true,
                min_binding_size: None,
            },
            count: None,
        }]
    }

    fn get_uniform_buffer(&self) -> &wgpu::Buffer {
        &self.uniform_buffer
    }

    fn apply_on_renderpass<'a>(
        &'a self,
        renderpass: &mut wgpu::RenderPass<'a>,
        write_queue: &wgpu::Queue,
    ) {
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        renderpass.set_index_buffer(self.index_buffer.slice(..));

        let instance_buffer_data: Vec<ObjectUniform> =
            self.objects.iter().map(Object::get_uniform_data).collect();

        let raw_data = unsafe {
            std::slice::from_raw_parts(
                instance_buffer_data.as_ptr() as *const u8,
                instance_buffer_data.len() * std::mem::size_of::<ObjectUniform>(),
            )
        };

        write_queue.write_buffer(self.get_uniform_buffer(), 0, raw_data);
        renderpass.draw_indexed(0..self.n_indexes, 0, 0..instance_buffer_data.len() as _);
    }
}
