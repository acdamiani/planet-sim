use super::body::Body;
use glam::f64::DVec3;

const G: f64 = 6.67430e-11;

pub struct System {
    bodies: Vec<Body>,
}

impl System {
    pub(super) fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub(super) fn insert(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub(super) fn step(&mut self, step: f64) {
        for i in 0..self.bodies.len() {
            let body = &self.bodies[i];

            let r = body.position();
            let v = body.velocity();
            let half_step = step / 2.0;
            let sixth_step = step / 6.0;

            let k1v = self.acceleration(body, DVec3::ZERO);
            let k1r = v;
            let k2v = self.acceleration(body, half_step * k1r);
            let k2r = half_step * v * k1v;
            let k3v = self.acceleration(body, half_step * k2r);
            let k3r = half_step * v * k2v;
            let k4v = self.acceleration(body, step * k3r);
            let k4r = step * v * k3v;

            self.bodies[i].apply(
                r + sixth_step * (k1r + 2.0 * k2r + 2.0 * k3r + k4r),
                v + sixth_step * (k1v + 2.0 * k2v + 2.0 * k3v + k4v),
            );
        }

        for body in self.bodies.iter_mut() {
            body.advance();
        }
    }

    fn acceleration(&self, body: &Body, offset: DVec3) -> DVec3 {
        let mut acceleration = DVec3::ZERO;
        let position = body.position() + offset;

        for other_body in &self.bodies {
            if body.id() == other_body.id() {
                continue;
            }

            let x = other_body.position() - position;
            acceleration += G * other_body.mass() / x.length().powi(3) * x;
        }

        acceleration
    }
}
