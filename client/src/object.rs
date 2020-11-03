use crate::uniform::{Uniform, UniformDescriptor};
use bytemuck;
use cgmath::{Matrix4, Quaternion, Vector3, InnerSpace};
use wgpu::{self, util::DeviceExt};

/*--------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
pub struct ObjectUniform {
    rotation: Matrix4<f32>,
    model: Matrix4<f32>,
    color: Vector3<f32>,
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

    pub fn rotate(&mut self, axis: &Vector3<f32>, angle: &cgmath::Deg<f32>) {
        let sin_angle_half = cgmath::Angle::sin(angle / 2.0);
        let cos_angle_half = cgmath::Angle::cos(angle / 2.0);
        let delta_rotation = Quaternion::new(
            cos_angle_half,
            sin_angle_half * axis.x,
            sin_angle_half * axis.y,
            sin_angle_half * axis.z,
        );
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
                label: Some("Model Uniform Buffer"),
                size: ObjectUniform::size(),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            }),

            objects: Vec::new(),
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
    fn get_binding(&self) -> u32 {
        1
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
        for obj in self.objects.iter() {
            let obj_data = obj.get_uniform_data();
            self.write_uniform(write_queue, &obj_data);
            renderpass.draw_indexed(0..self.n_indexes, 0, 0..1);
        }
    }
}
