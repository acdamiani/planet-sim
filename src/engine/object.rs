use super::material;
use slotmap::new_key_type;

new_key_type! { pub struct EngineKey; }

pub struct EngineObject {
    material: material::Material,
}

impl EngineObject {
    pub fn new(material: material::Material) -> Self {
        Self { material }
    }

    pub fn mat(&self) -> &material::Material {
        &self.material
    }
}
