use super::{
    mesh::{self, MeshDescriptor},
    renderer,
};
use glam::{Mat4, Quat, Vec2, Vec3};
use slotmap::new_key_type;
use wgpu::util::DeviceExt;

new_key_type! { pub struct EngineKey; }

pub struct EngineObject {
    mesh: mesh::MeshDescriptor,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

pub struct Instance {
    position: Vec2,
    rotation: Quat,
    color: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    mat: [[f32; 4]; 4],
    color: [f32; 3],
}

impl Instance {
    pub fn new(position: Vec2, rotation: Quat, color: Vec3) -> Self {
        Self {
            position,
            rotation,
            color,
        }
    }

    fn as_raw(&self) -> InstanceRaw {
        InstanceRaw {
            mat: Mat4::from_rotation_translation(self.rotation, self.position.extend(0.0))
                .to_cols_array_2d(),
            color: self.color.into(),
        }
    }

    pub fn data(instances: &[Instance]) -> Box<[u8]> {
        let instance_data = instances.iter().map(Instance::as_raw).collect::<Vec<_>>();
        bytemuck::cast_slice(&instance_data).into()
    }

    pub fn buffer(instances: &[Instance], device: &wgpu::Device) -> wgpu::Buffer {
        let data = Instance::data(instances);
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: &data,
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Mat row 0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Mat row 1
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Mat row 2
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Mat row 3
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Instance color
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

impl EngineObject {
    pub fn new(mesh: mesh::Mesh, instances: Vec<Instance>, device: &wgpu::Device) -> Self {
        let buffer = Instance::buffer(&instances, device);
        let desc = MeshDescriptor::new(mesh, device);
        Self {
            mesh: desc,
            instances,
            instance_buffer: buffer,
        }
    }

    pub fn odc(&self) -> Box<dyn renderer::Odc + '_> {
        let instance_buffer = self.instance_buffer.slice(..);
        let vertex_buffer = self.mesh.vertex_buffer().slice(..);

        match self.mesh.index_buffer() {
            Some(i) => Box::new(renderer::IndexedDc::new(
                vertex_buffer,
                instance_buffer,
                i.slice(..),
                self.mesh.indices(),
                // TODO: Better storage of instance buffer len w/ recreation on add/remove
                0..self.instances.len() as u32,
                None,
                None,
            )),
            None => Box::new(renderer::VertexDc::new(
                vertex_buffer,
                instance_buffer,
                self.mesh.vertices(),
                0..self.instances.len() as u32,
            )),
        }
    }

    pub fn mesh(&self) -> &mesh::MeshDescriptor {
        &self.mesh
    }

    pub fn instances(&self) -> &[Instance] {
        &self.instances
    }
}
