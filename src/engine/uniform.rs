use super::cam::Camera2D;
use std::sync::atomic::AtomicUsize;
use wgpu::util::DeviceExt;

static UB_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct UniformBufferBinding {
    id: usize,
    buffer: wgpu::Buffer,
    data: Vec<u8>,
    group: wgpu::BindGroup,
    group_layout: wgpu::BindGroupLayout,
}

impl UniformBufferBinding {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.group_layout
    }

    pub fn len(&self) -> u64 {
        self.data.len() as u64
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait UniformBuffer {
    fn bind(&self, device: &wgpu::Device) -> UniformBufferBinding;
    fn get_data(&self) -> Vec<u8>;
}

pub struct CameraBuffer<'a> {
    camera: &'a Camera2D,
}

impl<'a> CameraBuffer<'a> {
    pub fn new(camera: &'a Camera2D) -> Self {
        Self { camera }
    }
}

impl UniformBuffer for CameraBuffer<'_> {
    fn bind(&self, device: &wgpu::Device) -> UniformBufferBinding {
        let data = self.get_data();
        let group_layout = self.create_bind_group_layout(device);
        let buffer = self.create_buffer(device, &data);
        let group = self.create_bind_group(device, &buffer, &group_layout);

        UniformBufferBinding {
            id: UB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            buffer,
            data,
            group,
            group_layout,
        }
    }

    fn get_data(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[self.camera.build_proj_mat()]).to_vec()
    }
}

impl CameraBuffer<'_> {
    fn create_buffer(&self, device: &wgpu::Device, contents: &[u8]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            label: Some("camera_bind_group"),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    fn create_bind_group_layout(&self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        })
    }
}
