use super::mesh::MeshDrawCallBuilder;
use super::object::{EngineKey, EngineObject};
use super::renderer;
use slotmap::HopSlotMap;

pub struct Scene {
    // https://docs.rs/slotmap/latest/slotmap/#choosing-slotmap-hopslotmap-or-denseslotmap
    // `HopSlotMap` has a ~2x as slow insertion and deletion times compared to `SlotMap`
    // Iteration is significantly faster, however
    engine_objects: HopSlotMap<EngineKey, EngineObject>,
}

impl Scene {
    pub fn new() -> Self {
        let sm: HopSlotMap<EngineKey, EngineObject> = HopSlotMap::with_capacity_and_key(1024);

        Self { engine_objects: sm }
    }

    pub fn issue_key(&mut self, object: EngineObject) -> EngineKey {
        self.engine_objects.insert(object)
    }

    pub fn describe<'a>(
        &'a self,
        key: EngineKey,
        renderer: &renderer::Renderer,
        pass: &mut wgpu::RenderPass<'a>,
    ) -> MeshDrawCallBuilder {
        let obj = &self.engine_objects[key];
        pass.set_pipeline(obj.mat().pipeline());
        obj.mesh().draw(&renderer)
    }

    pub fn objects(&self) -> &HopSlotMap<EngineKey, EngineObject> {
        &self.engine_objects
    }
}
