use super::{mesh::MeshDrawCallBuilder, scene};

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
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
        }
    }

    pub fn render(&self, scene: &scene::Scene) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

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

        for (key, _value) in scene.objects() {
            builders.push(scene.describe(key, &self, &mut pass));
        }

        for call in &builders {
            Self::parse_draw_call(call, &mut pass);
        }

        drop(pass);
        self.queue.submit(std::iter::once(encoder.finish()));

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
