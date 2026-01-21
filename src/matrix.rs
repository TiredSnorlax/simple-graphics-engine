use std::ops::Sub;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn normalize(&mut self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

pub fn vec_sub(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

pub type Mat4x4 = [[f32; 4]; 4];

pub fn projection_matrix(aspect_ratio: f32, fov: f32, near: f32, far: f32) -> Mat4x4 {
    let fov = fov / 180.0 * std::f32::consts::PI;

    let f = 1.0 / (fov / 2.0).tan();

    let mut mat = [[0f32; 4]; 4];

    mat[0][0] = f / aspect_ratio;
    mat[1][1] = f;
    mat[2][2] = (far + near) / (far - near);
    mat[2][3] = 1.0;
    mat[3][2] = -(far * near) / (far - near);

    mat
}

pub fn rotate_x(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = 1.0;
    mat[1][1] = a.cos();
    mat[1][2] = a.sin();
    mat[2][1] = -a.sin();
    mat[2][2] = a.cos();
    mat[3][3] = 1.0;
    mat
}

pub fn rotate_y(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = a.cos();
    mat[0][2] = -a.sin();
    mat[1][1] = 1.0;
    mat[2][0] = a.sin();
    mat[2][2] = a.cos();
    mat[3][3] = 1.0;
    mat
}

pub fn rotate_z(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = a.cos();
    mat[0][1] = a.sin();
    mat[1][0] = -a.sin();
    mat[1][1] = a.cos();
    mat[2][2] = 1.0;
    mat[3][3] = 1.0;
    mat
}

pub fn translate(x: f32, y: f32, z: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = 1.0;
    mat[1][1] = 1.0;
    mat[2][2] = 1.0;
    mat[3][3] = 1.0;
    mat[3][0] = x;
    mat[3][1] = y;
    mat[3][2] = z;
    mat
}

pub fn mult_vec_mat(vec: &Vector3, mat: Mat4x4) -> Vector3 {
    let mut result = Vector3::new(0.0, 0.0, 0.0);

    result.x = vec.x * mat[0][0] + vec.y * mat[1][0] + vec.z * mat[2][0] + mat[3][0];
    result.y = vec.x * mat[0][1] + vec.y * mat[1][1] + vec.z * mat[2][1] + mat[3][1];
    result.z = vec.x * mat[0][2] + vec.y * mat[1][2] + vec.z * mat[2][2] + mat[3][2];
    let w = vec.x * mat[0][3] + vec.y * mat[1][3] + vec.z * mat[2][3] + mat[3][3];

    if w != 0.0 {
        result.x /= w;
        result.y /= w;
        result.z /= w;
    }
    result
}

pub fn cross_product(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
    Vector3::new(
        vec1.y * vec2.z - vec1.z * vec2.y,
        vec1.z * vec2.x - vec1.x * vec2.z,
        vec1.x * vec2.y - vec1.y * vec2.x,
    )
}

pub fn dot_product(vec1: &Vector3, vec2: &Vector3) -> f32 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}
