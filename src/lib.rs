mod matrix;
mod mesh;

// Re-export for the main file to use
pub use matrix::Vector3;

pub use crate::mesh::Mesh;

pub const FOV: f32 = 90.0;
pub const NEAR: f32 = 0.1;
pub const FAR: f32 = 100.0;

pub struct Object {
    pub mesh: Mesh,
    pub position: Vector3,
    pub rotation: Vector3,
}

impl Object {
    pub fn cube(position: Vector3) -> Self {
        let mesh = Mesh::cube();
        let rotation = Vector3::new(0.0, 0.0, 0.0);
        Object {
            mesh,
            position,
            rotation,
        }
    }

    pub fn draw(&self, width: f32, height: f32, camera: &Vector3, light_direction: &Vector3) {
        self.mesh.draw(
            width,
            height,
            &self.rotation,
            &self.position,
            camera,
            light_direction,
        );
    }
}
