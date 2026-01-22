#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z, w: 1.0 }
    }

    pub fn normalize(&mut self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3::new(self.x / length, self.y / length, self.z / length)
    }
}

pub fn vec_add(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
}

pub fn vec_sub(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z)
}

pub fn vec_add_a(v1: &Vector3, a: f32) -> Vector3 {
    Vector3::new(v1.x + a, v1.y + a, v1.z + a)
}

pub fn vec_sub_a(v1: &Vector3, a: f32) -> Vector3 {
    Vector3::new(v1.x - a, v1.y - a, v1.z - a)
}

pub fn vec_div(v1: &Vector3, divisor: f32) -> Vector3 {
    Vector3::new(v1.x / divisor, v1.y / divisor, v1.z / divisor)
}

pub fn vec_mul(v1: &Vector3, mult: f32) -> Vector3 {
    Vector3::new(v1.x * mult, v1.y * mult, v1.z * mult)
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

pub type Mat4x4 = [[f32; 4]; 4];

pub fn mat_multiply(mat1: &Mat4x4, mat2: &Mat4x4) -> Mat4x4 {
    let mut result = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    result
}

pub fn projection_matrix(aspect_ratio: f32, fov: f32, near: f32, far: f32) -> Mat4x4 {
    let fov = fov / 180.0 * std::f32::consts::PI;
    let f = 1.0 / (fov / 2.0).tan();

    let mut mat = [[0f32; 4]; 4];

    mat[0][0] = f / aspect_ratio;
    mat[1][1] = f;
    mat[2][2] = -(far + near) / (far - near);
    mat[2][3] = -1.0;
    mat[3][2] = -(2.0 * far * near) / (far - near);

    mat
}

pub fn rotate_x(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = 1.0;
    mat[1][1] = a.cos();
    mat[1][2] = -a.sin();
    mat[2][1] = a.sin();
    mat[2][2] = a.cos();
    mat[3][3] = 1.0;
    mat
}

pub fn rotate_y(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = a.cos();
    mat[0][2] = a.sin();
    mat[1][1] = 1.0;
    mat[2][0] = -a.sin();
    mat[2][2] = a.cos();
    mat[3][3] = 1.0;
    mat
}

pub fn rotate_z(a: f32) -> Mat4x4 {
    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = a.cos();
    mat[0][1] = -a.sin();
    mat[1][0] = a.sin();
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

pub fn point_at_mat(pos: &Vector3, target: &Vector3, up: &Vector3) -> Mat4x4 {
    let forward = (vec_sub(target, pos)).normalize();

    let a = vec_mul(&forward, dot_product(up, &forward));
    let new_up = vec_sub(up, &a).normalize();

    let mut mat = [[0.0; 4]; 4];
    mat[0][0] = right.x;
    mat[0][1] = right.y;
    mat[0][2] = right.z;
    mat[0][3] = -right.dot(pos);

    mat[1][0] = up.x;
    mat[1][1] = up.y;
    mat[1][2] = up.z;
    mat[1][3] = -up.dot(pos);

    mat[2][0] = forward.x;
    mat[2][1] = forward.y;
    mat[2][2] = forward.z;
    mat[2][3] = -forward.dot(pos);

    mat[3][3] = 1.0;
    mat
}

pub fn mult_vec_mat(vec: &Vector3, mat: Mat4x4) -> Vector3 {
    let mut result = Vector3::new(0.0, 0.0, 0.0);

    result.x = vec.x * mat[0][0] + vec.y * mat[1][0] + vec.z * mat[2][0] + vec.w * mat[3][0];
    result.y = vec.x * mat[0][1] + vec.y * mat[1][1] + vec.z * mat[2][1] + vec.w * mat[3][1];
    result.z = vec.x * mat[0][2] + vec.y * mat[1][2] + vec.z * mat[2][2] + vec.w * mat[3][2];
    result.w = vec.x * mat[0][3] + vec.y * mat[1][3] + vec.z * mat[2][3] + vec.w * mat[3][3];

    result
}
