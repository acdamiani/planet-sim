use std::ops::Range;

use glam::{Vec2, Vec3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
    uv: Vec2,
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Option<Vec<u16>>,
}

pub struct MeshDescriptor {
    vertex_buffer: wgpu::Buffer,
    index_buffer: Option<wgpu::Buffer>,
    vertices: u32,
    indices: u32,
}

impl MeshDescriptor {
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }

    pub fn vertices(&self) -> Range<u32> {
        0..self.vertices
    }

    pub fn indices(&self) -> Range<u32> {
        0..self.indices
    }
}

impl MeshDescriptor {
    pub fn new(mesh: Mesh, device: &wgpu::Device) -> Self {
        let (vertex_buffer, vertices) = (
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            mesh.vertices.len() as u32,
        );

        let (index_buffer, indices) = mesh.indices.map_or((None, 0), |i| {
            (
                Some(
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&i),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                ),
                i.len() as u32,
            )
        });

        Self {
            vertex_buffer,
            vertices,
            index_buffer,
            indices,
        }
    }
}

pub struct Quad {
    pub size: Vec2,
}

impl Default for Quad {
    fn default() -> Self {
        Self { size: Vec2::ONE }
    }
}

impl Quad {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

// https://github.com/bevyengine/bevy/blob/main/crates/bevy_render/src/mesh/shape/mod.rs#L157
impl From<Quad> for Mesh {
    fn from(value: Quad) -> Self {
        let extent_x = value.size.x / 2.0;
        let extent_y = value.size.y / 2.0;

        let vertices = vec![
            Vertex {
                position: Vec3::new(-extent_x, -extent_y, 0.0),
                uv: Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: Vec3::new(-extent_x, extent_y, 0.0),
                uv: Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: Vec3::new(extent_x, extent_y, 0.0),
                uv: Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(extent_x, -extent_y, 0.0),
                uv: Vec2::new(1.0, 1.0),
            },
        ];

        let indices = vec![0, 2, 1, 0, 3, 2];

        Self {
            vertices,
            indices: Some(indices),
        }
    }
}

pub struct Tri;

impl Tri {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Tri {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Tri> for Mesh {
    fn from(_value: Tri) -> Self {
        let vertices = vec![
            Vertex {
                position: Vec3::new(0.0, 0.5, 0.0),
                uv: Vec2::new(0.5, 0.0),
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.0),
                uv: Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.0),
                uv: Vec2::new(1.0, 1.0),
            },
        ];

        Mesh {
            vertices,
            indices: None,
        }
    }
}
