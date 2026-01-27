use macroquad::{
    input::{KeyCode, is_key_down},
    time::get_frame_time,
};

use crate::{Vector3, matrix::*};
const CAMERA_SPEED: f32 = 15.0;

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
        let rotation_x = rotate_x(self.rotation_x);
        let rotation_mat = mat_multiply(&rotation_x, &rotation_y);
        mult_vec_mat(&front, &rotation_mat)
    }

    pub fn right(&self) -> Vector3 {
        let right = Vector3::new(1.0, 0.0, 0.0);
        let rotation_y = rotate_y(self.rotation_y);
        let rotation_x = rotate_x(self.rotation_x);
        let rotation_mat = mat_multiply(&rotation_x, &rotation_y);
        mult_vec_mat(&right, &rotation_mat)
    }

    pub fn up(&self) -> Vector3 {
        let up = Vector3::new(0.0, 1.0, 0.0);
        let rotation_y = rotate_y(self.rotation_y);
        let rotation_x = rotate_x(self.rotation_x);
        let rotation_mat = mat_multiply(&rotation_x, &rotation_y);
        mult_vec_mat(&up, &rotation_mat)
    }

    pub fn return_view_mat(&self) -> [[f32; 4]; 4] {
        let target = vec_add(&self.position, &self.direction());
        let point_at = point_at_mat(&self.position, &target, &self.up);
        let look_at = quick_inverse_mat(&point_at);
        look_at
    }

    pub fn handle_user_input(&mut self) {
        let delta = get_frame_time();

        let forward = vec_mul(&self.direction(), CAMERA_SPEED * delta);
        // Rotation of camera
        if is_key_down(KeyCode::Up) {
            self.rotation_x += CAMERA_SPEED / 10.0 * delta;
        }
        if is_key_down(KeyCode::Down) {
            self.rotation_x -= CAMERA_SPEED / 10.0 * delta;
        }
        if is_key_down(KeyCode::Left) {
            self.rotation_y -= CAMERA_SPEED / 10.0 * delta;
        }
        if is_key_down(KeyCode::Right) {
            self.rotation_y += CAMERA_SPEED / 10.0 * delta;
        }

        // Movement of camera
        if is_key_down(KeyCode::W) {
            self.position = vec_add(&self.position, &forward);
        }
        if is_key_down(KeyCode::S) {
            self.position = vec_sub(&self.position, &forward);
        }
        if is_key_down(KeyCode::A) {
            // Move left relative to camera's direction
            self.position = vec_add(
                &self.position,
                &vec_mul(&self.right(), CAMERA_SPEED * delta),
            );
        }
        if is_key_down(KeyCode::D) {
            self.position = vec_add(
                &self.position,
                &vec_mul(&self.right(), -CAMERA_SPEED * delta),
            );
        }
        if is_key_down(KeyCode::Space) {
            self.position = vec_add(&self.position, &vec_mul(&self.up(), CAMERA_SPEED * delta));
        }
        if is_key_down(KeyCode::LeftShift) {
            self.position = vec_add(&self.position, &vec_mul(&self.up(), -CAMERA_SPEED * delta));
        }
    }
}
