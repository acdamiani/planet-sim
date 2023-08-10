pub struct Shader {
    name: &'static str,
    source: &'static str,
}

impl Shader {
    pub fn new(name: &'static str, source: &'static str) -> Self {
        Self { name, source }
    }

    pub(super) fn bind(&self, device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(self.name),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(self.source)),
        })
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn source(&self) -> &'static str {
        self.source
    }
}
