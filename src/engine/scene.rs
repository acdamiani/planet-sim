use std::borrow::Cow;

use super::cam::Camera2D;
use super::mesh::{Mesh, Quad};
use super::object::{EngineKey, EngineObject, Instance, InstanceRaw};
use super::renderer;
use crate::sim::Sim;
use glam::{Vec2, Vec3};
use slotmap::HopSlotMap;

pub struct Scene {
    // https://docs.rs/slotmap/latest/slotmap/#choosing-slotmap-hopslotmap-or-denseslotmap
    // `HopSlotMap` has a ~2x as slow insertion and deletion times compared to `SlotMap`
    // Iteration is significantly faster, however
    engine_objects: HopSlotMap<EngineKey, EngineObject>,
    camera: Camera2D,
    sim: Sim,
    sim_key: EngineKey,
    grad: colorous::Gradient,
}

impl Scene {
    pub fn new(renderer: &renderer::Renderer) -> Self {
        let mut sm: HopSlotMap<EngineKey, EngineObject> = HopSlotMap::with_capacity_and_key(1024);
        let sim = Sim::new();
        let instances = sim
            .system()
            .bodies()
            .iter()
            .map(|b| {
                Instance::new(
                    b.position().as_vec3(),
                    glam::Quat::from_rotation_z(0.0),
                    glam::Vec3::new(1.0, 0.0, 0.0),
                )
            })
            .collect::<Vec<Instance>>();

        let sim_key = sm.insert(EngineObject::new(
            Mesh::from(Quad::new(0.25 * Vec2::ONE)),
            instances,
            renderer.device(),
        ));

        let camera = Camera2D::new(
            1.0,
            0.0,
            Vec2::ZERO,
            Vec2::new(
                renderer.config().width as f32,
                renderer.config().height as f32,
            ),
        );

        Self {
            engine_objects: sm,
            camera,
            sim,
            sim_key,
            grad: colorous::VIRIDIS,
        }
    }

    // TODO: Move to util mod
    fn rgb_vec(r: u8, g: u8, b: u8) -> Vec3 {
        Vec3::new(
            r as f32 / u8::MAX as f32,
            g as f32 / u8::MAX as f32,
            b as f32 / u8::MAX as f32,
        )
    }

    pub fn step_sim(&mut self, dt: f64) -> (EngineKey, &EngineObject) {
        self.sim.step(dt);

        let object = &mut self.engine_objects[self.sim_key];
        for (i, b) in object
            .instances_mut()
            .iter_mut()
            .zip(self.sim.system().bodies())
        {
            let color = self
                .grad
                .eval_continuous(b.velocity().length_squared() / 100.0);

            i.position = b.position().as_vec3();
            i.color = Self::rgb_vec(color.r, color.g, color.b);
        }

        (self.sim_key, object)
    }

    pub fn issue_key(&mut self, object: EngineObject) -> EngineKey {
        self.engine_objects.insert(object)
    }

    pub fn objects(&self) -> &HopSlotMap<EngineKey, EngineObject> {
        &self.engine_objects
    }

    pub fn objects_mut(&mut self) -> &mut HopSlotMap<EngineKey, EngineObject> {
        &mut self.engine_objects
    }

    pub fn camera(&self) -> &Camera2D {
        &self.camera
    }
}
