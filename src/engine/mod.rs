use self::{
    buffer::{CameraBuffer, UniformBuffer},
    cam::Camera2D,
    mesh::*,
    object::EngineObject,
    pipeline::PipelineBuilder,
    shader::Shader,
};
use anyhow::{Context, Result};
use glam::Vec2;
use std::vec;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct Engine {
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    window: window::Window,
    renderer: Option<renderer::Renderer>,
}

impl Engine {
    pub fn new() -> Result<Self> {
        env_logger::init();

        // Create event loop and window
        let event_loop = EventLoop::new();
        let window = window::Window::new(&event_loop, Some("Planet Sim"))
            .context("Failed to initialize window")?;

        // Initialize wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        Ok(Self {
            event_loop,
            instance,
            window,
            renderer: None,
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        let surface =
            unsafe { self.instance.create_surface(&self.window.window()) }.with_context(|| {
                format!(
                    "Failed to create surface on window {}",
                    &self.window.window().title()
                )
            })?;

        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("Adapter request failed")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .context("Device request failed")?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_fmt = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_fmt,
            width: self.window.size().width,
            height: self.window.size().height,
            // TODO: Different present modes
            // let modes = &surface_caps.present_modes;
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        self.renderer = Some(renderer::Renderer::new(
            surface,
            device,
            queue,
            surface_config,
        ));

        Ok(())
    }

    pub fn begin_loop(mut self) -> Result<()> {
        let mut renderer = self
            .renderer
            .context("init must be called before beginning the loop")?;

        let camera = Camera2D::new(
            100.0,
            0.0,
            Vec2::ZERO,
            Vec2::new(
                renderer.config().width as f32,
                renderer.config().height as f32,
            ),
        );

        let camera_buffer = CameraBuffer::new(camera).bind(renderer.device());

        let _shader = renderer.bind_shader(Shader::new(
            "s_wireframe",
            include_str!("../../assets/shaders/wire.wgsl"),
        ));

        let pipeline_builder = PipelineBuilder::new(pipeline::PipelineType::Wireframe)
            .with_bind_group_layouts(Box::new([camera_buffer.bind_group_layout()]));
        let pipeline = pipeline_builder.build(renderer.device(), &renderer)?;

        let mut scene = scene::Scene::new(&renderer);

        let obj = EngineObject::new(Mesh::from(Quad::default()));
        scene.issue_key(obj);

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.window().id() => {
                    // TODO: Input handler goes here
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(size) => {
                            match self.window.resize(renderer.config(), *size) {
                                Some(config) => renderer.configure(config),
                                None => {}
                            }
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.window().id() => {
                    // TODO: Update method goes here
                    match renderer.render(&scene, &pipeline, &[camera_buffer.bind_group()]) {
                        Ok(_) => {}
                        Err(error) => {
                            if let Some(surface_error) = error.downcast_ref::<wgpu::SurfaceError>()
                            {
                                match surface_error {
                                    wgpu::SurfaceError::Lost => {
                                        self.window.resize(renderer.config(), self.window.size());
                                    }
                                    wgpu::SurfaceError::OutOfMemory => {
                                        *control_flow = ControlFlow::Exit;
                                    }
                                    e => eprintln!("{:?}", e),
                                };
                            };
                        }
                    }
                }
                Event::MainEventsCleared => self.window.window().request_redraw(),
                _ => {}
            });
    }
}

pub mod buffer;
pub mod cam;
pub mod mesh;
pub mod object;
pub mod pipeline;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod window;
