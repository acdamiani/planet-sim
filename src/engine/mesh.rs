use crate::engine::renderer;
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

impl Mesh {
    pub fn draw<'a>(&'a self, device: &wgpu::Device) -> MeshDrawCallBuilder {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mut draw_call =
            MeshDrawCallBuilder::new(vertex_buffer, self.vertices.len().try_into().unwrap());

        match &self.indices {
            Some(ibuf) => {
                draw_call = draw_call.with_indices(
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&ibuf),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                    ibuf.len().try_into().unwrap(),
                )
            }
            None => {}
        }

        draw_call
    }
}

pub struct MeshDrawCallBuilder {
    vertex_buffer: wgpu::Buffer,
    vertices_count: u32,
    index_buffer: Option<wgpu::Buffer>,
    indices_count: Option<u32>,
}

// TODO: Instancing
impl MeshDrawCallBuilder {
    pub fn new(vertex_buffer: wgpu::Buffer, len: u32) -> Self {
        Self {
            vertex_buffer,
            vertices_count: len,
            index_buffer: None,
            indices_count: None,
        }
    }

    pub fn with_indices(mut self, buffer: wgpu::Buffer, len: u32) -> Self {
        self.index_buffer = Some(buffer);
        self.indices_count = Some(len);
        self
    }

    pub fn v_buf(&self) -> (&wgpu::Buffer, u32) {
        (&self.vertex_buffer, self.vertices_count)
    }

    pub fn i_buf(&self) -> Option<(&wgpu::Buffer, u32)> {
        match &self.index_buffer {
            Some(i) => Some((i, self.indices_count.unwrap())),
            None => None,
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

        Self {
            vertices,
            indices: None,
        }
    }
}
