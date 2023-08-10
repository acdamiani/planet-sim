use super::cam::Camera2D;
use super::mesh::MeshDrawCallBuilder;
use super::object::{EngineKey, EngineObject};
use super::{buffer, renderer};
use glam::Vec2;
use slotmap::HopSlotMap;

pub struct Scene {
    // https://docs.rs/slotmap/latest/slotmap/#choosing-slotmap-hopslotmap-or-denseslotmap
    // `HopSlotMap` has a ~2x as slow insertion and deletion times compared to `SlotMap`
    // Iteration is significantly faster, however
    engine_objects: HopSlotMap<EngineKey, EngineObject>,
    camera: Camera2D,
}

impl Scene {
    pub fn new(renderer: &renderer::Renderer) -> Self {
        let sm: HopSlotMap<EngineKey, EngineObject> = HopSlotMap::with_capacity_and_key(1024);

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
        }
    }

    pub fn issue_key(&mut self, object: EngineObject) -> EngineKey {
        self.engine_objects.insert(object)
    }

    pub fn describe<'a>(
        &'a self,
        key: EngineKey,
        renderer: &renderer::Renderer,
    ) -> MeshDrawCallBuilder {
        let obj = &self.engine_objects[key];
        obj.mesh().draw(&renderer)
    }

    pub fn objects(&self) -> &HopSlotMap<EngineKey, EngineObject> {
        &self.engine_objects
    }

    pub fn camera(&self) -> &Camera2D {
        &self.camera
    }
}
