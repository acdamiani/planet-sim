use winit::{error::OsError, event_loop::EventLoop, window::WindowBuilder};

pub struct Window {
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>, window_name: Option<&str>) -> Result<Self, OsError> {
        let name = match window_name {
            Some(name) => name,
            None => "New Window",
        };

        let window = WindowBuilder::new().with_title(name).build(&event_loop)?;
        let size = window.inner_size();

        Ok(Self { window, size })
    }

    pub fn resize(
        &mut self,
        surface_config: wgpu::SurfaceConfiguration,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) -> Option<wgpu::SurfaceConfiguration> {
        let mut config = surface_config;

        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            config.width = self.size.width;
            config.height = self.size.height;
            Some(config)
        } else {
            None
        }
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }
}
