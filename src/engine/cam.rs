use glam::{Mat4, Quat, Vec2, Vec3};

pub struct Camera3D {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera3D {
    pub fn new<V: Into<Vec3>>(position: V, yaw: f32, pitch: f32) -> Self {
        Self {
            position: position.into(),
            yaw,
            pitch,
        }
    }

    pub fn mat(&self) -> Mat4 {
        let pitch_sin = self.pitch.sin();
        let pitch_cos = self.pitch.cos();

        let yaw_sin = self.pitch.sin();
        let yaw_cos = self.pitch.cos();

        Mat4::look_to_rh(
            self.position,
            Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize(),
            Vec3::Y,
        )
    }
}

pub struct Projection {
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fov_y: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov_y,
            z_far,
            z_near,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn mat(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect, self.z_near, self.z_far)
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }
}

pub struct Camera2D {
    pub scale: f32,
    pub rotation: f32,
    pub translation: Vec2,
    pub screen_size: Vec2,
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
