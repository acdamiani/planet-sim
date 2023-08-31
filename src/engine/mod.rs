use crate::app::App;

use self::{
    mesh::*,
    object::{EngineKey, EngineObject, Instance},
    pipeline::PipelineBuilder,
    shader::Shader,
    uniform::{CameraBuffer, UniformBuffer},
};
use anyhow::{Context, Result};
use glam::{Quat, Vec2, Vec3};
use std::{time::Instant, vec};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct Engine {
    app: Option<App>,
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
            app: None,
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

        let renderer = renderer::Renderer::new(surface, device, queue, surface_config);

        let config = renderer.config();
        self.app = Some(App::new(Vec2::new(
            config.width as f32,
            config.height as f32,
        )));

        self.renderer = Some(renderer);

        Ok(())
    }

    pub fn begin_loop(mut self) -> Result<()> {
        let mut renderer = self
            .renderer
            .context("init must be called before beginning the loop")?;
        let mut app = self.app.unwrap();

        let camera_binding = CameraBuffer::new(app.camera()).bind(renderer.device());

        let _shader = renderer.bind_shader(Shader::new(
            "s_solid",
            include_str!("../../assets/shaders/default.wgsl"),
        ));

        let pipeline_builder = PipelineBuilder::new(pipeline::PipelineType::Solid)
            .with_bind_group_layouts(&[camera_binding.bind_group_layout()]);
        let pipeline = pipeline_builder.build(renderer.device(), &renderer)?;

        let mut scene = scene::Scene::new(&renderer);

        let mut last_render = Instant::now();

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.window().id() => {
                    if !app.input(event) {
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
                                if let Some(config) = self.window.resize(renderer.config(), *size) {
                                    app.resize(Vec2::new(
                                        config.width as f32,
                                        config.height as f32,
                                    ));
                                    renderer.configure(config);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.window().id() => {
                    let now = Instant::now();
                    let dt = now - last_render;
                    last_render = now;

                    renderer.request_buffer_update(
                        camera_binding.id(),
                        &app.update_and_build_controller(dt),
                    );

                    let (k, o) = scene.step_sim(dt.as_secs_f64() / 12.0);
                    renderer.instance_buffer_update(
                        k,
                        bytemuck::cast_slice(
                            &o.instances()
                                .iter()
                                .map(Instance::as_raw)
                                .collect::<Vec<_>>(),
                        ),
                    );

                    match renderer.render(&scene, &pipeline, &camera_binding) {
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

pub mod cam;
pub mod mesh;
pub mod object;
pub mod pipeline;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod uniform;
pub mod window;
