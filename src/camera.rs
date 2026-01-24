use macroquad::input::{KeyCode, is_key_down};

use crate::{Vector3, matrix::*};
const CAMERA_SPEED: f32 = 0.15;

pub struct Camera {
    pub position: Vector3,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub up: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn direction(&self) -> Vector3 {
        let front = Vector3::new(0.0, 0.0, 1.0);
        let rotation_y = rotate_y(self.rotation_y);
        mult_vec_mat(&front, &rotation_y)
    }

    pub fn return_view_mat(&self) -> [[f32; 4]; 4] {
        let target = vec_add(&self.position, &self.direction());
        let point_at = point_at_mat(&self.position, &target, &self.up);
        let look_at = quick_inverse_mat(&point_at);
        look_at
    }

    pub fn handle_user_input(&mut self) {
        let forward = vec_mul(&self.direction(), CAMERA_SPEED);
        // Rotation of camera
        if is_key_down(KeyCode::Up) {
            self.position.y += CAMERA_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            self.position.y -= CAMERA_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.position.x += CAMERA_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            self.position.x -= CAMERA_SPEED;
        }

        // Movement of camera
        if is_key_down(KeyCode::W) {
            self.position = vec_add(&self.position, &forward);
        }
        if is_key_down(KeyCode::S) {
            self.position = vec_sub(&self.position, &forward);
        }
        if is_key_down(KeyCode::A) {
            self.rotation_y -= CAMERA_SPEED / 10.0;
        }
        if is_key_down(KeyCode::D) {
            self.rotation_y += CAMERA_SPEED / 10.0;
        }
    }
}
