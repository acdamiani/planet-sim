use anyhow::{Context, Result};

use super::{
    mesh::{self, Vertex},
    renderer::Renderer,
};

pub enum PipelineType {
    Solid,
    Wireframe,
}

pub struct PipelineBuilder<'a> {
    pipeline: PipelineType,
    label: Option<&'static str>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    push_constant_ranges: Vec<wgpu::PushConstantRange>,
    shader: Option<wgpu::ShaderModule>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(pipeline: PipelineType) -> Self {
        Self {
            pipeline,
            label: None,
            bind_group_layouts: vec![],
            push_constant_ranges: vec![],
            shader: None,
        }
    }

    pub fn with_layout_label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_bind_group_layouts(mut self, layouts: Box<[&'a wgpu::BindGroupLayout]>) -> Self {
        self.bind_group_layouts = layouts.to_vec();
        self
    }

    pub fn with_push_constant_ranges(mut self, ranges: Box<[wgpu::PushConstantRange]>) -> Self {
        self.push_constant_ranges = ranges.to_vec();
        self
    }

    pub fn with_shader(mut self, shader: wgpu::ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn build(self, device: &wgpu::Device, renderer: &Renderer) -> Result<wgpu::RenderPipeline> {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(self.label.unwrap_or("Pipeline Layout")),
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &self.push_constant_ranges,
        });

        let shader = match self.shader {
            Some(ref s) => anyhow::Ok(s),
            None => match self.pipeline {
                PipelineType::Solid => renderer
                    .get_shader_by_name("s_solid")
                    .context("The default solid shader \"s_solid\" was not inserted before pipeline build"),
                PipelineType::Wireframe => renderer
                    .get_shader_by_name("s_wireframe")
                    .context("The default wireframe shader \"s_wireframe\" was not inserted before pipeline build"),
            },
        }?;

        match self.pipeline {
            PipelineType::Solid => Ok(device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some("Solid Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: shader,
                        entry_point: "vs_main",
                        buffers: &[mesh::Vertex::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: renderer.config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires
                        // Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                },
            )),
            PipelineType::Wireframe => Ok(device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some("Wireframe Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: renderer.config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::LineStrip,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires
                        // Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                },
            )),
        }
    }
}
