use crate::engine::cam::Camera2D;
use glam::Vec2;
use std::time::Duration;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent},
};

pub struct CameraController2D {
    min_zoom: f32,
    max_zoom: f32,
    zoom_speed: f32,
    pan_speed: f32,
    axis_x: [f32; 2],
    axis_y: [f32; 2],
    pan_target: Vec2,
    rotation_target: f32,
    axis_rotation: [f32; 2],
    d_scale: f32,
}

impl CameraController2D {
    pub fn new(zoom_speed: f32, pan_speed: f32, min_zoom: f32, max_zoom: f32) -> Self {
        Self {
            zoom_speed,
            pan_speed,
            min_zoom,
            max_zoom,
            axis_x: [0.0, 0.0],
            axis_y: [0.0, 0.0],
            d_scale: 0.0,
            pan_target: Vec2::ZERO,
            rotation_target: 0.0,
            axis_rotation: [0.0, 0.0],
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    self.d_scale = -y * self.zoom_speed;
                    true
                }

                MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                    self.d_scale = -*y as f32 * self.zoom_speed;
                    true
                }
            },
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    virtual_keycode,
                    state,
                    ..
                } => {
                    if let Some(v) = virtual_keycode {
                        self.handle_keys(*v, *state)
                    } else {
                        false
                    }
                }
            },
            _ => false,
        }
    }

    fn handle_keys(&mut self, keycode: VirtualKeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        match keycode {
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.axis_x[0] = amount;
                true
            }
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.axis_y[1] = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Back => {
                self.axis_y[0] = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.axis_x[1] = amount;
                true
            }
            VirtualKeyCode::Q => {
                self.axis_rotation[0] = amount;
                true
            }
            VirtualKeyCode::E => {
                self.axis_rotation[1] = amount;
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self, camera: &mut Camera2D, dt: Duration) -> Vec<u8> {
        let dt = dt.as_secs_f32();

        let new_zoom = camera.scale + self.d_scale * dt;
        camera.scale = new_zoom.clamp(self.min_zoom, self.max_zoom);

        self.pan_target += Vec2::new(
            self.axis_x[1] - self.axis_x[0],
            self.axis_y[1] - self.axis_y[0],
        ) * 500.0
            / camera.scale
            * dt;

        let new_pan = camera.translation.lerp(self.pan_target, 0.1);
        camera.translation = new_pan;

        self.rotation_target += (self.axis_rotation[1] - self.axis_rotation[0]) * 5.0 * dt % 360.0;
        let new_rotation = Self::f_lerp(camera.rotation, self.rotation_target, 0.1);
        camera.rotation = new_rotation;

        bytemuck::cast_slice(&[camera.build_proj_mat()]).to_vec()
    }

    fn f_lerp(a: f32, b: f32, f: f32) -> f32 {
        a * (1.0 - f) + (b * f)
    }
}

impl Default for CameraController2D {
    fn default() -> Self {
        Self {
            zoom_speed: 50.0,
            pan_speed: 10.0,
            axis_x: [0.0, 0.0],
            axis_y: [0.0, 0.0],
            d_scale: 0.0,
            min_zoom: 10.0,
            max_zoom: 1000.0,
            pan_target: Vec2::ZERO,
            axis_rotation: [0.0, 0.0],
            rotation_target: 0.0,
        }
    }
}
