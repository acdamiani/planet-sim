use glam::{Mat4, Quat, Vec2};

pub struct Camera2D {
    scale: f32,
    rotation: f32,
    translation: Vec2,
    screen_size: Vec2,
}

impl Camera2D {
    pub fn new(scale: f32, rotation: f32, translation: Vec2, screen_size: Vec2) -> Self {
        Self {
            scale,
            rotation,
            translation,
            screen_size,
        }
    }

    pub fn build_proj_mat(&self) -> Mat4 {
        let view = Mat4::from_rotation_translation(
            Quat::from_rotation_z(self.rotation),
            self.translation.extend(1.0),
        );

        let (w, h) = self.screen_size.into();
        let half_w = w / 2.0;
        let half_h = h / 2.0;

        let proj = Mat4::orthographic_rh(
            -half_w / self.scale,
            half_w / self.scale,
            -half_h / self.scale,
            half_h / self.scale,
            0.0,
            1.0,
        );

        proj * view.inverse()
    }
}
