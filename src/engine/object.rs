use super::mesh;
use slotmap::new_key_type;

new_key_type! { pub struct EngineKey; }

pub struct EngineObject {
    mesh: mesh::Mesh,
}

impl EngineObject {
    pub fn new(mesh: mesh::Mesh) -> Self {
        Self { mesh }
    }

    pub fn mesh(&self) -> &mesh::Mesh {
        &self.mesh
    }
}
