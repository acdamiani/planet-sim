use super::{scene, shader::Shader, uniform::UniformBufferBinding};
use anyhow::{anyhow, Context, Ok, Result};
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    ops::Range,
};
use wgpu::{BufferSize, BufferSlice};

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    staging_belt: wgpu::util::StagingBelt,
    shaders: HashMap<&'static str, wgpu::ShaderModule>,
    buffer_update: VecDeque<(usize, Box<[u8]>)>,
}

impl Renderer {
    pub fn new(
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
    ) -> Self {
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            // WARN: StagingBelt chunk size should change when/if new uniform buffers are
            // implemented
            staging_belt: wgpu::util::StagingBelt::new(0x60),
            shaders: HashMap::default(),
            buffer_update: VecDeque::default(),
        }
    }

    pub fn request_buffer_update(&mut self, id: usize, data: &[u8]) {
        self.buffer_update.push_back((id, data.into()));
    }

    pub fn render(
        &mut self,
        scene: &scene::Scene,
        pipeline: &wgpu::RenderPipeline,
        camera_buffer: &UniformBufferBinding,
    ) -> Result<()> {
        let output = self
            .surface
            .get_current_texture()
            .context("Failed to retrieve next swapchain texture target")?;
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        while let Some(v) = self.buffer_update.pop_front() {
            if v.0 == camera_buffer.id() {
                let mut buffer = self.staging_belt.write_buffer(
                    &mut encoder,
                    camera_buffer.buffer(),
                    0x0,
                    BufferSize::new(camera_buffer.len()).unwrap(),
                    &self.device,
                );

                buffer.copy_from_slice(v.1.as_ref());
            }
        }

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, camera_buffer.bind_group(), &[]);

        for (_key, value) in scene.objects() {
            let odc = value.odc();
            odc.draw(&mut pass);
        }

        drop(pass);

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        self.staging_belt.recall();

        output.present();

        Ok(())
    }

    pub fn bind_shader(&mut self, shader: Shader) -> Result<&wgpu::ShaderModule> {
        match self.shaders.entry(shader.name()) {
            Entry::Occupied(_) => Err(anyhow!("Shader \"{}\" is already bound", shader.name())),
            Entry::Vacant(e) => {
                let binding = shader.bind(&self.device);
                Ok(e.insert(binding))
            }
        }
    }

    pub fn get_shader_by_name(&self, name: &'static str) -> Option<&wgpu::ShaderModule> {
        self.shaders.get(name)
    }

    pub fn configure(&mut self, new_config: wgpu::SurfaceConfiguration) {
        self.config = new_config;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn config(&self) -> wgpu::SurfaceConfiguration {
        self.config.clone()
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
}

pub trait Odc<'a> {
    // WARN: lifetime spaghetti
    // Quick explanation bc lifetimes are weird:
    // We're declaring a lifetime `'a` on trait `Odc`
    // In the draw method, we're assering (using the `where` keyword)
    // that `'a` outlives `'r`, the lifetime of the `RenderPass`
    // We declare it on the trait so that we can use it to tell rustc
    // that any struct that implements this trait and uses this lifetime param
    // as a generic argument outlives the `RenderPass`, satisfying wgpu's lifetime
    // requirements
    fn draw<'r>(&self, pass: &mut wgpu::RenderPass<'r>)
    where
        'a: 'r;
}

pub struct VertexDc<'a> {
    vertex_buffer: BufferSlice<'a>,
    instance_buffer: BufferSlice<'a>,
    vertices: Range<u32>,
    instances: Range<u32>,
}

impl<'a> VertexDc<'a> {
    pub fn new(
        vertex_buffer: BufferSlice<'a>,
        instance_buffer: BufferSlice<'a>,
        vertices: Range<u32>,
        instances: Range<u32>,
    ) -> Self {
        Self {
            vertex_buffer,
            instance_buffer,
            vertices,
            instances,
        }
    }
}

impl<'a> Odc<'a> for VertexDc<'a> {
    fn draw<'r>(&self, pass: &mut wgpu::RenderPass<'r>)
    where
        'a: 'r,
    {
        pass.set_vertex_buffer(0, self.vertex_buffer);
        pass.set_vertex_buffer(1, self.instance_buffer);
        pass.draw(self.vertices.clone(), self.instances.clone());
    }
}

pub struct IndexedDc<'a> {
    vertex_buffer: BufferSlice<'a>,
    instance_buffer: BufferSlice<'a>,
    index_buffer: BufferSlice<'a>,
    indices: Range<u32>,
    instances: Range<u32>,
    format: Option<wgpu::IndexFormat>,
    base_vertex: Option<i32>,
}

impl<'a> IndexedDc<'a> {
    pub fn new(
        vertex_buffer: BufferSlice<'a>,
        instance_buffer: BufferSlice<'a>,
        index_buffer: BufferSlice<'a>,
        indices: Range<u32>,
        instances: Range<u32>,
        format: Option<wgpu::IndexFormat>,
        base_vertex: Option<i32>,
    ) -> Self {
        Self {
            vertex_buffer,
            instance_buffer,
            index_buffer,
            indices,
            instances,
            format,
            base_vertex,
        }
    }
}

impl<'a> Odc<'a> for IndexedDc<'a> {
    fn draw<'r>(&self, pass: &mut wgpu::RenderPass<'r>)
    where
        'a: 'r,
    {
        pass.set_vertex_buffer(0, self.vertex_buffer);
        pass.set_vertex_buffer(1, self.instance_buffer);
        pass.set_index_buffer(
            self.index_buffer,
            self.format.unwrap_or(wgpu::IndexFormat::Uint16),
        );
        pass.draw_indexed(
            self.indices.clone(),
            self.base_vertex.unwrap_or(0),
            self.instances.clone(),
        );
    }
}
