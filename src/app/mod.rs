use self::controller::CameraController;
use crate::engine::cam::{Camera3D, Projection};
use std::time::Duration;
use winit::event::{KeyboardInput, WindowEvent};

pub struct App {
    camera: Camera3D,
    projection: Projection,
    camera_controller: CameraController,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            camera: Camera3D::new(
                (0.0, 0.0, 0.0),
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
            ),
            projection: Projection::new(width, height, (45.0_f32).to_radians(), 0.1, 100.0),
            camera_controller: CameraController::new(4.0, 0.4),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.projection.resize(width, height)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
    }

    pub fn uniform_data(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[self.projection.mat() * self.camera.mat()]).to_vec()
    }

    pub fn camera(&self) -> &Camera3D {
        &self.camera
    }
}

pub mod controller;
