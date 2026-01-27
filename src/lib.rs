mod camera;
pub mod matrix;
mod mesh;

// Re-export for the main file to use
pub use crate::camera::Camera;
pub use crate::mesh::Mesh;
use macroquad::texture::Image;
pub use matrix::Vector3;

pub const FOV: f32 = 90.0;
pub const NEAR: f32 = 0.1;
pub const FAR: f32 = 100.0;

pub struct Object {
    pub mesh: Mesh,
    pub position: Vector3,
    pub rotation: Vector3,
    pub texture: Option<Image>,
}

impl Object {
    // pub fn cube(position: Vector3) -> Self {
    //     let mesh = Mesh::cube();
    //     let rotation = Vector3::new(0.0, 0.0, 0.0);
    //     Object {
    //         mesh,
    //         position,
    //         rotation,
    //     }
    // }

    pub fn draw(
        &self,
        width: f32,
        height: f32,
        camera: &Camera,
        light_direction: &Vector3,
        projection_mat: &matrix::Mat4x4,
        view_mat: &matrix::Mat4x4,
        image: &mut Image,
        depth_buffer: &mut Vec<f32>,
    ) {
        self.mesh.draw(
            width,
            height,
            &self.rotation,
            &self.position,
            &view_mat,
            &camera.position,
            light_direction,
            projection_mat,
            image,
            &self.texture,
            depth_buffer,
        );
    }
}
