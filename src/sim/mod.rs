use glam::DVec3;

use self::{body::BodyBuilder, system::System};

pub struct Sim {
    system: System,
}

impl Sim {
    // TODO: Init from config file
    pub fn new() -> Self {
        let mut system = System::new();

        // 1 solar mass (sun)
        let mut body = BodyBuilder::new(1.0)
            .with_position(DVec3::new(0.0, 0.0, 0.0))
            .build();
        system.insert(body);
        // Earth
        body = BodyBuilder::new(3e-6)
            .with_position(DVec3::new(0.0, -0.98329, 0.0))
            .with_velocity(DVec3::new(6.38966, 0.0, 0.0))
            .build();
        system.insert(body);

        Self { system }
    }

    pub fn step(&mut self, dt: f64) {
        self.system.step(dt);
    }

    pub fn system(&self) -> &System {
        &self.system
    }
}

pub mod body;
pub mod system;
