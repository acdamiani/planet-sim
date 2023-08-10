use self::controller::CameraController2D;
use crate::engine::cam::Camera2D;
use glam::Vec2;
use std::time::Duration;
use winit::event::WindowEvent;

pub struct App {
    camera: Camera2D,
    camera_controller: CameraController2D,
}

impl App {
    pub fn new(screen: Vec2) -> Self {
        Self {
            camera: Camera2D::new(100.0, 0.0, Vec2::ZERO, screen),
            camera_controller: CameraController2D::default(),
        }
    }

    pub fn resize(&mut self, size: Vec2) {
        self.camera.screen_size = size;
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.input(event)
    }

    pub fn update_and_build_controller(&mut self, dt: Duration) -> Vec<u8> {
        self.camera_controller.update(&mut self.camera, dt)
    }

    pub fn camera(&self) -> &Camera2D {
        &self.camera
    }
}

pub mod controller;
