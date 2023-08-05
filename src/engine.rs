use anyhow::{Context, Result};
use std::vec;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use self::{material::Material, object::EngineObject};

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

        let mut scene = scene::Scene::new();
        let obj = EngineObject::new(Material::new(renderer.device(), renderer.config().format));

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
                    match renderer.render(&scene) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            self.window.resize(renderer.config(), self.window.size());
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => self.window.window().request_redraw(),
                _ => {}
            });
    }
}

pub mod material;
pub mod object;
pub mod renderer;
pub mod scene;
pub mod window;
