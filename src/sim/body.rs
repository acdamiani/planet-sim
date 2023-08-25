use glam::f64::DVec3;
use std::sync::atomic::AtomicUsize;

static BODY_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Body {
    id: usize,

    position: DVec3,
    velocity: DVec3,
    mass: f64,

    n_pos: DVec3,
    n_vel: DVec3,
}

impl Body {
    pub fn apply(&mut self, position: DVec3, velocity: DVec3) {
        self.n_pos = position;
        self.n_vel = velocity;
    }

    pub fn advance(&mut self) {
        self.position = self.n_pos;
        self.velocity = self.n_vel;
    }

    pub fn position(&self) -> DVec3 {
        self.position
    }

    pub fn velocity(&self) -> DVec3 {
        self.velocity
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct BodyBuilder {
    mass: f64,
    position: Option<DVec3>,
    velocity: Option<DVec3>,
}

impl BodyBuilder {
    pub fn new(mass: f64) -> Self {
        Self {
            mass,
            position: None,
            velocity: None,
        }
    }

    pub fn with_position(mut self, position: DVec3) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_velocity(mut self, velocity: DVec3) -> Self {
        self.velocity = Some(velocity);
        self
    }

    pub fn build(&self) -> Body {
        let position = self.position.unwrap_or(DVec3::ZERO);
        let velocity = self.velocity.unwrap_or(DVec3::ZERO);

        Body {
            id: BODY_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            position,
            velocity,
            mass: self.mass,
            n_pos: position,
            n_vel: velocity,
        }
    }
}
