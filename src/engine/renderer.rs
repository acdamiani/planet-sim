use super::{
    buffer::{CameraBuffer, UniformBufferBinding},
    cam::Camera2D,
    mesh::MeshDrawCallBuilder,
    scene,
    shader::Shader,
};
use anyhow::{anyhow, Context, Ok, Result};
use glam::Vec2;
use std::collections::{hash_map::Entry, HashMap, VecDeque};
use wgpu::{BufferSize, DynamicOffset};

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

        let mut builders: Vec<MeshDrawCallBuilder> = Vec::with_capacity(scene.objects().len());

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

        for (key, _value) in scene.objects() {
            builders.push(scene.describe(key, &self.device));
        }

        for call in &builders {
            Self::parse_draw_call(call, &mut pass);
        }

        drop(pass);

        self.staging_belt.finish();

        self.queue.submit(std::iter::once(encoder.finish()));

        self.staging_belt.recall();

        output.present();

        Ok(())
    }

    fn parse_draw_call<'a>(call: &'a MeshDrawCallBuilder, pass: &mut wgpu::RenderPass<'a>) {
        let (vbuf, lvbuf) = call.v_buf();
        let oibuf = call.i_buf();

        pass.set_vertex_buffer(0, vbuf.slice(..));

        match oibuf {
            Some((ibuf, libuf)) => {
                pass.set_index_buffer(ibuf.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..libuf, 0, 0..1);
            }
            None => {
                pass.draw(0..lvbuf, 0..1);
            }
        }
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
