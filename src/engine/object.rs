use super::{material, mesh};
use slotmap::new_key_type;

new_key_type! { pub struct EngineKey; }

pub struct EngineObject {
    mesh: mesh::Mesh,
    material: material::Material,
}

impl EngineObject {
    pub fn new(mesh: mesh::Mesh, material: material::Material) -> Self {
        Self { mesh, material }
    }

    pub fn mat(&self) -> &material::Material {
        &self.material
    }

    pub fn mesh(&self) -> &mesh::Mesh {
        &self.mesh
    }
}
